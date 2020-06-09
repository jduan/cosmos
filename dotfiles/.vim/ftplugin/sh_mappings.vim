" automatically give executable permissions when saving shell scripts
autocmd BufWritePost * silent !chmod a+x <afile>
