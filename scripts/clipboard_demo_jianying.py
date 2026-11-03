import os
import sys


current_dir = os.path.dirname(os.path.abspath(__file__))
env_root = os.getenv("JY_SKILL_ROOT", "").strip()
skill_candidates = [
    env_root,
    os.path.join(current_dir, ".agent", "skills", "jianying-editor"),
    os.path.join(current_dir, ".trae", "skills", "jianying-editor"),
    os.path.join(current_dir, ".claude", "skills", "jianying-editor"),
    os.path.join(current_dir, "skills", "jianying-editor"),
    os.path.abspath(".agent/skills/jianying-editor"),
    os.path.dirname(current_dir),
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
        break

if not scripts_path:
    raise ImportError(
        "Could not find jianying-editor/scripts/jy_wrapper.py\nTried:\n- "
        + "\n- ".join(attempted)
    )

if scripts_path not in sys.path:
    sys.path.insert(0, scripts_path)

from jy_wrapper import JyProject
import pyJianYingDraft as draft


PROJECT_ROOT = os.path.dirname(current_dir)
VIDEO_PATH = os.path.join(
    PROJECT_ROOT,
    "docs",
    "marketing",
    "assets",
    "clipboard-ui-demo-landscape-v1.mp4",
)


def add_caption(project: JyProject, text: str, start: str, duration: str) -> None:
    project.add_text_simple(
        text,
        start_time=start,
        duration=duration,
        track_name="Feature captions",
        style=draft.TextStyle(size=5.8, bold=True, color=(1.0, 1.0, 1.0), align=1),
        border=draft.TextBorder(alpha=0.28, width=18.0),
        background=draft.TextBackground(
            color="#1D1C1A",
            alpha=0.82,
            round_radius=0.55,
            height=0.12,
            width=0.38,
        ),
        clip_settings=draft.ClipSettings(transform_y=-0.82),
    )


def main() -> None:
    if not os.path.exists(VIDEO_PATH):
        raise FileNotFoundError(VIDEO_PATH)

    project = JyProject("Clipboard_UI_Demo_Landscape", width=1920, height=1080)
    if project.add_media_safe(VIDEO_PATH, "0s", track_name="UI demo") is None:
        raise RuntimeError("视频素材导入失败")

    add_caption(project, "搜索刚刚复制过的内容", "0s", "8s")
    add_caption(project, "选中需要的句子，再复制为 Markdown", "8s", "11s")
    add_caption(project, "图片历史完整保留，随时找回", "19s", "12s")
    project.save()


if __name__ == "__main__":
    main()
