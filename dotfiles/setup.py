#!/usr/bin/env python3
import os
import shutil
import subprocess

EXCLUDE_LIST = [".", "..", ".git"]
DRY_RUN = False

HOME = os.environ['HOME']
target_dir = os.path.abspath(os.path.dirname(__file__))
src_dir = HOME + "/"
os.chdir(target_dir)

# Iterate the "dotfiles" dir and create symlinks from HOME to the files there
for file in os.listdir(target_dir):
    if file.startswith(".") and file not in EXCLUDE_LIST:
        src_path = os.path.join(src_dir, file)
        target_path = os.path.join(target_dir, file)
        if os.path.islink(src_path) and os.path.exists(src_path):
            if DRY_RUN:
                print(f"DRY RUN: src_path {src_path} exists, removing it now!")
            else:
                print(f"WARNING: src_path {src_path} exists, removing it now!")
                os.remove(src_path)
        if DRY_RUN:
            print(f"DRY RUN: creating symlink: from {target_path} to {src_path}")
        else:
            print(f"creating symlink: from {target_path} to {src_path}")
            subprocess.run(['ln', '-s', target_path, src_path])

# Create ~/.vim/swap/ directory to store swap files. This directory has to exist for vim to use it.
vim_swap_dir = os.path.join(os.path.expanduser("~"), ".vim", "swap")
print(f"creating vim swap dir: {vim_swap_dir}")
if not DRY_RUN:
    os.makedirs(vim_swap_dir, exist_ok=True)

# Create ~/.ssh if it doesn't exist yet
ssh_dir = f"{HOME}/.ssh"
if not os.path.isdir(ssh_dir):
    os.makedirs(ssh_dir)
    os.chmod(ssh_dir, 0o755)

# Additional things
additional_mappings = {
    f"{HOME}/.ssh/config": f"{target_dir}/.ssh_config",
}
for src, target in additional_mappings.items():
    if DRY_RUN:
        print(f"DRY RUN: creating symlink from {src} to {target}")
    else:
        print(f"creating symlink from {src} to {target}")
        subprocess.run(['ln', '-s', target, src])
