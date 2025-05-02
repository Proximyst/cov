setup-precommit:
    echo "#!/bin/sh" > .git/hooks/pre-commit
    echo "set -e" >> .git/hooks/pre-commit
    echo "just _precommit" >> .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

_precommit:
    zizmor .
    actionlint
    goimports -l .