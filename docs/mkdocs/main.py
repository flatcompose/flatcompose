"""
MkDocs macros module
"""
import os


DEFAULTS = {
    "FLATCOMPOSE_REPO_URL":    "https://github.com/flatcompose/flatcompose",
    "FLATCOMPOSE_RELEASE_URL": "https://github.com/flatcompose/flatcompose/releases/latest/download",
    "FLATIMAGE_RELEASE_URL":   "https://github.com/flatimage/flatimage/releases/latest/download",
}


def define_env(env):
    for var, default in DEFAULTS.items():
        env.variables[var] = os.environ.get(var) or default
