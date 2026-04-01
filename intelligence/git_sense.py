import subprocess
import time

_GIT_CACHE = {
    "branch": "Unknown",
    "modified_count": 0,
    "last_check": 0
}

CACHE_TTL = 5 # 5 seconds

def get_git_status():
    """
    Returns the current git branch and modified file count.
    Results are cached for CACHE_TTL seconds to avoid overhead.
    """
    now = time.time()
    if now - _GIT_CACHE["last_check"] < CACHE_TTL:
        return _GIT_CACHE["branch"], _GIT_CACHE["modified_count"]

    try:
        # Get branch name
        branch = subprocess.check_output(
            ["git", "rev-parse", "--abbrev-ref", "HEAD"],
            stderr=subprocess.STDOUT,
            text=True
        ).strip()
        
        # Get modified file count
        status = subprocess.check_output(
            ["git", "status", "--porcelain"],
            stderr=subprocess.STDOUT,
            text=True
        ).strip()
        
        modified_count = len(status.splitlines()) if status else 0
        
        _GIT_CACHE["branch"] = branch
        _GIT_CACHE["modified_count"] = modified_count
        _GIT_CACHE["last_check"] = now
        
        return branch, modified_count
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "No Git", 0

if __name__ == "__main__":
    print(get_git_status())
