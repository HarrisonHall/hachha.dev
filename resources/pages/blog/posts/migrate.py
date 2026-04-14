#!/usr/bin/env python

from pathlib import Path


def foo():
    cwd = Path(".")
    for dir in cwd.iterdir():
        print(dir)
        if not dir.is_dir():
            continue
        blog_name: str = dir.name

        print(blog_name)

        blog_data = dir / "blog.toml"
        blog_content = dir / "blog.md"

        if not blog_data.is_file() or not blog_content.is_file():
            continue

        fm = blog_data.read_text()
        content = blog_content.read_text()

        new_content = f"+++\n{fm}+++\n\n{content}"

        out = dir.parent / f"{blog_name}.md"
        out.write_text(new_content)

        blog_data.unlink()
        blog_content.unlink()

        try:
            dir.rmdir()
        except Exception:
            pass


if __name__ == "__main__":
    foo()
