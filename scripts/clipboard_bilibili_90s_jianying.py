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
ASSETS_DIR = os.path.join(PROJECT_ROOT, "docs", "marketing", "assets")


def asset(name: str) -> str:
    path = os.path.join(ASSETS_DIR, name)
    if not os.path.exists(path):
        raise FileNotFoundError(path)
    return path


def add_scene(
    project: JyProject,
    name: str,
    start: str,
    duration: str,
    scale: float = 1.0,
) -> None:
    segment = project.add_media_safe(
        asset(name),
        start_time=start,
        duration=duration,
        track_name="Main story",
    )
    if segment is None:
        raise RuntimeError(f"素材导入失败：{name}")
    segment.clip_settings = draft.ClipSettings(scale_x=scale, scale_y=scale)


def add_caption(project: JyProject, text: str, start: str, duration: str) -> None:
    project.add_text_simple(
        text,
        start_time=start,
        duration=duration,
        track_name="Narrative captions",
        style=draft.TextStyle(size=5.7, bold=True, color=(1.0, 1.0, 1.0), align=1),
        border=draft.TextBorder(alpha=0.24, width=16.0),
        background=draft.TextBackground(
            color="#1D1C1A",
            alpha=0.84,
            round_radius=0.55,
            height=0.12,
            width=0.46,
        ),
        clip_settings=draft.ClipSettings(transform_y=-0.82),
    )


def main() -> None:
    project = JyProject("Clipboard_Bilibili_90s", width=1920, height=1080)

    add_scene(project, "scene-01-hero.png", "0s", "8s")
    add_scene(project, "scene-02-download-trust.png", "8s", "12s")
    add_scene(project, "clipboard-ui-demo-landscape-v1.mp4", "20s", "35.52s")
    add_scene(project, "scene-04-passwords.png", "55.52s", "12.48s")
    add_scene(project, "scene-05-privacy-policy.png", "68s", "14s")
    add_scene(project, "scene-06-download-cta.png", "82s", "8s")

    captions = [
        ("复制一次，不该丢掉上一次。", "0s", "4.5s"),
        ("Clipboard 把常用内容留在本机。", "4.5s", "3.5s"),
        ("官网公开版本、系统要求与安装包校验值。", "8s", "6s"),
        ("当前版本支持 64 位 Windows。", "14s", "6s"),
        ("先搜索，再按分类找到需要的内容。", "20s", "10s"),
        ("只选需要的句子，再复制为 Markdown。", "30s", "13s"),
        ("文字和图片历史，可以随时切换。", "43s", "12.52s"),
        ("密码库在本地加密保存。", "55.52s", "12.48s"),
        ("当前版本不需要账户，也没有云同步。", "68s", "14s"),
        ("从官网或 GitHub Releases 下载。", "82s", "8s"),
    ]
    for text, start, duration in captions:
        add_caption(project, text, start, duration)

    project.save()


if __name__ == "__main__":
    main()
