function generate_all_sources
    set git_root (git rev-parse --show-toplevel)
    $git_root/scripts/generate_all_sources.sh
end
