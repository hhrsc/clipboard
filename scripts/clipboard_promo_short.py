import os
import shutil
import subprocess
import sys

current_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(current_dir)
env_root = os.getenv("JY_SKILL_ROOT", "").strip()
skill_candidates = [
    env_root,
    os.path.join(project_root, ".agent", "skills", "jianying-editor"),
    os.path.join(project_root, ".trae", "skills", "jianying-editor"),
    os.path.join(project_root, ".claude", "skills", "jianying-editor"),
    os.path.join(project_root, "skills", "jianying-editor"),
    os.path.abspath(".agent/skills/jianying-editor"),
    r"C:\Users\Lenovo\.codex\skills\jianying-editor-skill",
]

scripts_path = None
attempted = []
for candidate in skill_candidates:
    if not candidate:
        continue
    candidate = os.path.abspath(candidate)
    attempted.append(candidate)
    if os.path.exists(os.path.join(candidate, "scripts", "jy_wrapper.py")):
        scripts_path = os.path.join(candidate, "scripts")
        skill_root = candidate
        break

if not scripts_path:
    raise ImportError(
        "Could not find jianying-editor/scripts/jy_wrapper.py\nTried:\n- "
        + "\n- ".join(attempted)
    )

if scripts_path not in sys.path:
    sys.path.insert(0, scripts_path)

from jy_wrapper import JyProject


SCENES = [
    ("clipboard-recent.webp", "复制以后，依然找得到", "本地优先的 Windows 剪贴板工具"),
    ("clipboard-recent.webp", "只复制你真正需要的那一段", "纯文本、Markdown、HTML 或单行文本"),
    ("clipboard-images.webp", "文字和图片，都有历史", "搜索、分类、置顶，再一键复制回来"),
    ("clipboard-passwords.webp", "密码留在本机", "本地加密保存，当前版本没有云同步"),
    ("clipboard-settings.webp", "Clipboard for Windows", "hhrsc.github.io/website"),
]


def run(command: list[str]) -> None:
    print("Running:", command[0], command[-1])
    subprocess.run(command, check=True)


def filter_escape(value: str) -> str:
    return value.replace("\\", "/").replace(":", r"\:").replace("'", r"\'")


def build_video(ffmpeg: str, font_path: str, silent_output: str, final_output: str) -> None:
    marketing_dir = os.path.join(project_root, "static", "marketing")
    audio_path = os.path.join(skill_root, "assets", "audio.mp3")
    scene_duration = 4.4
    transition_duration = 0.6
    font = filter_escape(font_path)
    command = [ffmpeg, "-y"]
    for filename, _, _ in SCENES:
        command.extend(
            [
                "-loop",
                "1",
                "-framerate",
                "30",
                "-t",
                str(scene_duration),
                "-i",
                os.path.join(marketing_dir, filename),
            ]
        )

    filters = []
    for index, (_, title, subtitle) in enumerate(SCENES):
        title = filter_escape(title)
        subtitle = filter_escape(subtitle)
        filters.append(
            f"[{index}:v]split=2[bg{index}][fg{index}];"
            f"[bg{index}]scale=1080:1920:force_original_aspect_ratio=increase,"
            f"crop=1080:1920,gblur=sigma=46,eq=brightness=-0.14:saturation=0.65[bgc{index}];"
            f"[fg{index}]scale=1000:1000:force_original_aspect_ratio=decrease[fgc{index}];"
            f"[bgc{index}][fgc{index}]overlay=(W-w)/2:(H-h)/2,"
            f"drawtext=fontfile='{font}':text='{title}':x=(w-text_w)/2:y=170:"
            "fontsize=58:fontcolor=white:box=1:boxcolor=black@0.58:boxborderw=24,"
            f"drawtext=fontfile='{font}':text='{subtitle}':x=(w-text_w)/2:y=1690:"
            "fontsize=29:fontcolor=white:box=1:boxcolor=black@0.52:boxborderw=18,"
            f"trim=duration={scene_duration},setpts=PTS-STARTPTS[v{index}]"
        )

    current = "v0"
    for index in range(1, len(SCENES)):
        output = f"x{index}"
        offset = index * (scene_duration - transition_duration)
        filters.append(
            f"[{current}][v{index}]xfade=transition=fade:duration={transition_duration}:"
            f"offset={offset:.1f}[{output}]"
        )
        current = output
    filters.append(f"[{current}]fps=30,format=yuv420p[outv]")

    command.extend(
        [
            "-filter_complex",
            ";".join(filters),
            "-map",
            "[outv]",
            "-an",
            "-c:v",
            "libx264",
            "-preset",
            "medium",
            "-crf",
            "19",
            "-movflags",
            "+faststart",
            silent_output,
        ]
    )
    run(command)

    total_duration = len(SCENES) * scene_duration - (len(SCENES) - 1) * transition_duration
    run(
        [
            ffmpeg,
            "-y",
            "-i",
            silent_output,
            "-stream_loop",
            "-1",
            "-i",
            audio_path,
            "-t",
            str(total_duration),
            "-filter:a",
            "volume=0.32",
            "-map",
            "0:v:0",
            "-map",
            "1:a:0",
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-b:a",
            "192k",
            "-shortest",
            "-movflags",
            "+faststart",
            final_output,
        ]
    )


def create_draft(silent_video: str) -> str:
    audio_path = os.path.join(skill_root, "assets", "audio.mp3")
    project = JyProject(
        project_name="Clipboard_首发短视频_V1",
        width=1080,
        height=1920,
        overwrite=True,
    )
    video = project.add_media_safe(
        silent_video,
        start_time="0s",
        duration="19.6s",
        track_name="VideoTrack",
    )
    if video is None:
        raise RuntimeError("Video import failed.")

    for start, duration in [("0s", "5s"), ("5s", "5s"), ("10s", "5s"), ("15s", "4.6s")]:
        audio = project.add_audio_safe(
            audio_path,
            start_time=start,
            duration=duration,
            track_name="BGM_Track",
        )
        if audio is None:
            raise RuntimeError(f"Audio import failed at {start}.")
        audio.volume = 0.32

    result = project.save()
    return result.get("draft_path", project.draft_dir)


def main() -> None:
    ffmpeg = shutil.which("ffmpeg")
    if not ffmpeg:
        raise RuntimeError("ffmpeg is unavailable.")
    font_path = next(
        (
            path
            for path in [
                r"C:\Windows\Fonts\msyh.ttc",
                r"C:\Windows\Fonts\msyhbd.ttc",
                r"C:\Windows\Fonts\simhei.ttf",
            ]
            if os.path.exists(path)
        ),
        None,
    )
    if not font_path:
        raise RuntimeError("No supported Chinese font was found.")

    output_dir = os.path.join(project_root, "docs", "marketing", "assets")
    os.makedirs(output_dir, exist_ok=True)
    silent_output = os.path.join(output_dir, "clipboard-promo-vertical-v1-silent.mp4")
    final_output = os.path.join(output_dir, "clipboard-promo-vertical-v1.mp4")
    build_video(ffmpeg, font_path, silent_output, final_output)
    draft_path = create_draft(silent_output)
    print(f"PROMO_VIDEO={final_output}")
    print(f"JY_DRAFT={draft_path}")


if __name__ == "__main__":
    main()
