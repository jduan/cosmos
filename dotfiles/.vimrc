" https://github.com/junegunn/vim-plug
" Specify a directory for plugins
" - For Neovim: ~/.local/share/nvim/plugged
" - Avoid using standard Vim directory names like 'plugin'
call plug#begin('~/.vim/plugged')

" Plug 'Glench/Vim-Jinja2-Syntax'
" Plug 'eagletmt/ghcmod-vim'
" Plug 'eagletmt/neco-ghc'
" Plug 'idris-hackers/idris-vim'
" Plug 'mrk21/yaml-vim'

Plug 'Einenlum/yaml-revealer'
Plug 'Clavelito/indent-awk.vim'
" A lib that's used by other plugins
Plug 'MarcWeber/vim-addon-mw-utils'
" a great asynchronous execution library for Vim
Plug 'Shougo/vimproc.vim', {'do' : 'make'}
" display the indention levels with thin vertical lines
Plug 'Yggdroot/indentLine'
" Integrates with hindent so every time you save a Haskell source file it gets
" automatically prettified.
Plug 'alx741/vim-hindent'
" Full path fuzzy file, buffer, mru, tag, ... finder for Vim.
Plug 'ctrlpvim/ctrlp.vim'
" Vim support for editing fish scripts
Plug 'dag/vim-fish'
" 'Vim to Haskell': A collection of vimscripts for Haskell development
Plug 'dag/vim2hs'
" Vim motions on speed!
Plug 'easymotion/vim-easymotion'
" Elixir support for vim
Plug 'elixir-editors/vim-elixir'
" Perform all your vim insert mode completions with Tab
Plug 'ervandew/supertab'
" Like 'f', but press 's' and two characters
Plug 'goldfeld/vim-seek'
" basic vim/terraform integration
Plug 'hashivim/vim-terraform'
" A nicer Python indentation style for vim
Plug 'hynek/vim-python-pep8-indent'
Plug 'itchyny/vim-haskell-indent'
" insert or delete brackets, parens, quotes in pair
Plug 'jiangmiao/auto-pairs'
Plug 'jlanzarotta/bufexplorer'
Plug 'jpo/vim-railscasts-theme'
" You need the following 2 plugins together.
" Also this assumes 'fzf' is installed via homebrew.
Plug 'junegunn/fzf'
Plug 'junegunn/fzf.vim'
" Run your favorite search tool from Vim, with an enhanced results list.
" You can configure a different search tool other than 'ack'.
Plug 'mileszs/ack.vim'
" Unobtrusive scratch window.
Plug 'mtth/scratch.vim'
" Extended f, F, t and T key mappings for Vim.
Plug 'rhysd/clever-f.vim'
Plug 'scrooloose/nerdtree'
Plug 'scrooloose/syntastic'
Plug 'sheerun/vim-polyglot'
Plug 'solarnz/thrift.vim'
Plug 'thinca/vim-visualstar'
Plug 'tomtom/tcomment_vim'
Plug 'tomtom/tlib_vim'
Plug 'tpope/vim-endwise'
Plug 'tpope/vim-eunuch'
Plug 'tpope/vim-fireplace'
Plug 'tpope/vim-fugitive'
" GitHub extension for fugitive.vim
Plug 'tpope/vim-rhubarb'
Plug 'tpope/vim-rsi'
Plug 'tpope/vim-sleuth'
Plug 'tpope/vim-surround'
Plug 'tpope/vim-unimpaired'
Plug 'troydm/pb.vim'
Plug 'vim-scripts/PreserveNoEOL'
" Make a column of increasing or decreasing numbers, dates, or daynames.
Plug 'vim-scripts/VisIncr'
" Required by vim-session
Plug 'xolox/vim-misc'
Plug 'xolox/vim-session'
" shows how many times does a search pattern occur in the current buffer
Plug 'google/vim-searchindex'
" note-taking app
Plug 'alok/notational-fzf-vim'
" Plug 'JamshedVesuna/vim-markdown-preview'
Plug 'vim-scripts/groovy.vim'
Plug 'vim-scripts/groovyindent-unix'
" Close multiple buffers at once.
Plug 'Asheq/close-buffers.vim'
Plug 'rust-lang/rust.vim'
Plug 'LnL7/vim-nix'
Plug 'christoomey/vim-titlecase' " Titlecase text
" Run shell scripts against vim autocmd events
Plug 'ahw/vim-hooks'
" Mozart language
Plug 'Procrat/oz.vim'
Plug 'godlygeek/tabular'
" Initialize plugin system
call plug#end()

"""""""""""""""" vim's built-in package manager """"""""""""""""
packadd! matchit
"""""""""""""""" end """"""""""""""""

" disable 'highlighting the matching paren'
let loaded_matchparen=1

" swap files
" Use // at the end of the filename makes vim use absolute file paths for the
" swap file names so you don't get name collisions.
set directory^=$HOME/.vim/swap//


""""""""""""""""""" leader maps """""""""""""""""""
let mapleader = ","

" Insert the current git branch name. The branch naming convention is:
" jduan/IPD-26383-blah-blah
noremap <leader>b :read !git rev-parse --abbrev-ref HEAD <bar> cut -d/ -f2 <bar> cut -d"-" -f1,2 <cr>
" capslock.vim: turn caps on and off
" default behavior is to disable caps lock when leaving insert mode
imap <leader>c <Plug>CapsLockToggle
" Switch CWD to the directory of the open buffer:
map <leader>cd :cd %:p:h<cr>:pwd<cr>
" foreplay.vim
nmap <leader>e :Eval<cr>
nnoremap <leader>eb :EasyBuffer<CR>
nnoremap <leader>g :Git
nnoremap <leader>gs :Gstatus<CR>
nnoremap <leader>gb :Gblame<CR>
nnoremap <leader>gd :Gdiff<CR>
nnoremap <leader>gl :Glog<CR>
nnoremap <leader>gp :Git push<CR>
nnoremap <leader>gc :Gcommit
" gundo plugin
nnoremap <leader>gu :GundoToggle<CR>
map <Leader>n :NERDTreeToggle<CR>
map <leader>p :set paste<CR>
map <leader>pp :set nopaste<CR>
nnoremap <leader>q gqip
nnoremap <leader>t :Tabularize /
" make file read only
nnoremap <leader>ro :set nomodifiable<CR>
" <leader>W to remove all trailing white spaces
nnoremap <leader>rs :%s/\s\+$//<cr>:let @/=''<CR>
nnoremap <leader>rw :set modifiable<CR>
" open up a shell inside vim
nnoremap <leader>sh :ConqueTerm zsh<CR>
nnoremap <leader>sc :SyntasticToggleMode<CR>
" saveas to the path of the current file
command! -nargs=1 SaveasSamePath exe "sav " . expand("%:p:h") . "/" .  expand("<args>")
nmap <leader>ss :SaveasSamePath
nnoremap <leader>tl :TlistToggle<CR>
" tagbar
nnoremap <leader>tb :TagbarToggle<CR>
" in command mode, double leader to go to previous file
nnoremap <leader><leader> <c-^>
" switch to previous window in insert mode
inoremap <C-^> <Esc><C-^>
nnoremap <C-e> <C-^>
" C-e doesn't work because of vim-rsi plugin
inoremap <C-e> <C-^>
inoremap <C-h> <Left>
inoremap <C-l> <Right>
" Haskell: ->
inoremap <leader>a ->
inoremap <leader>aa =>
" fast save a buffer
nmap <leader>w :w<cr>
nmap <leader>wa :wa<cr>


set autoindent           " carry over indenting from previous line
set background=dark      " console bg is dark
set backspace=2          " allow backspace beyond insertion point
set backupdir-=.         " ...and not in cwd
set backupdir=/tmp       " put backups in /tmp
set cindent
set cinkeys-=0#          " don't unindent cpp stuff (perl comments!)
set clipboard=unnamed   " put yanks/etc on the clipboard
set colorcolumn=100       " show vertical line for column 100
set com+=s1:/*,mb:*,ex:*/  " ...and also C-style comments
set comments=b://,b:#    " by default allow C++ (JS) and generic unixy comments
set copyindent           " make autoindent use the same characters to indent
set cursorline            " turn on cursor line
set encoding=utf-8       " unicode
set expandtab            " spaces, not tabs!
set fileignorecase       " ignore file/directory cases when using file names and directories.
set foldcolumn=0         " no fold column
set nofoldenable         " disable folding entirely
set foldlevelstart=0     " start with all folds closed
set formatoptions=r      " r - re-insert comment leader on newline
set guioptions+=agimrLt  " make vim act like a gui when started like one
set hidden               " hide, don't close, undisplayed buffers
set history=50           " keep 50 lines of command history
set hlsearch             " highlight all matches for the pattern
set ignorecase smartcase " make searches case-sensitive only if they contain upper-case letters
set incsearch
set laststatus=2
set more                 " page on extended output
set nocompatible         " We're running Vim, not Vi!
set novb                 " disable visual bell
set number               " turn on line numbering
set pastetoggle=<S-F1>   " Shift-F1 to toggle paste
set path=.,$HOME,,       " for editing with :find
set relativenumber       " use relative number
set report=1             " Always report changes
set ruler                " display cursor position
set shiftwidth=4         " affects what happens when you press >>, << etc
set showcmd              " show command-in-progress
set showmatch            " Automagically show matching brackets
set showmode             " show the current input mode
set softtabstop=4        " make four spaces act like tabs
" set statusline=%<[%02n]\ %F%(\ %m%h%w%y%r%)\ %{fugitive#statusline()}\ %a%=\ [%l,%c%V](%P)\ [%08O:%02B]
set t_Co=256             " 256 colors
set tabstop=4            " The One True Tab (as of latest revision)
set terse
set textwidth=100
set timeout              " allow keys to timeout
set timeoutlen=500       " millisecs, time out between 2 keystrokes
set title                " do set the xterm title (see 'titleold', set below)
set undolevels=1000      " LOTS of undo history
set wildignore+=*/CVS/   " don't try to descend into CVS directories
set wildignore+=*/SVN/   " don't try to descend into SVN directories
set wildmenu             " enable menu of completions
set wildmode=longest,list " tab completion: longest only
set wrap                 " automatically wrap lines
set writeany             " avoid constant ! to overwrite. . .

" Clear the search buffer when hitting the "space bar"
function! MapCR()
  nnoremap <Space> :nohlsearch<cr>
endfunction"
call MapCR()

" Ack the word under cursor
" Use "rg" as ack
let g:ackprg = 'rg --vimgrep --no-heading'
" If you want to ignore certain files/dirs, edit '~/.ripgreprc' instead.
nnoremap <leader>a :Ack! <cword><CR>

" bufferexplorer: use relative path
nnoremap <leader>be :BufExplorer<CR>
let g:bufExplorerShowRelativePath=1
let g:bufExplorerSortBy='mru'


""""""""""""""" filetype association """""""""""""""
" python scripts
au BufNewFile,BufRead,BufReadPost *.mesos,*.aurora set filetype=python
au BufNewFile,BufRead,BufReadPost BUILD set filetype=python | let b:syntastic_mode = 'passive'
au BufNewFile,BufRead,BufReadPost *.bzl set filetype=python
" use clojure plugin to format Racket files
au BufNewFile,BufRead,BufReadPost *.rkt,*.rktl set filetype=clojure
au BufNewFile,BufRead,BufReadPost *.pipeline set filetype=groovy
" scratch files
au BufNewFile,BufRead,BufReadPost scratch,scratch2 set textwidth=0

" exec'd because ... could it work otherwise?
exec "set titleold=" . matchstr(hostname(), "^[^.]\\+")

" use a UTF-8 term if you can
if $TERM_PROGRAM == "Apple_Terminal"
  set tenc=utf-8
endif

"
" Arrow macros
"
map! ^K ^VESCO
map! ^VESC^B ESCbi
map! ^VESC^F ESCEa
map! ^VESCOA ^VESCka
map! ^VESCOB ^VESCja
map! ^VESCOC ^VESClli
map! ^VESCOD ^VESCi
map ^V^? x
map ^VESCOH 0
map ^VESCOF $
map ^VESC[H 0
map ^VESC[F $
map [H I
map [F A

"
" Use fonts that don't make my eyes bleed
"
if has("gui_running")
    if has("macunix")
        set guifont=ProFont:h9
    elseif has("win32")
        set guifont=ProFontWindows:h11:b:cANSI,Lucida_Console:h11:b:cANSI
    elseif has("x11")
        set guifont=-jmk-neep-bold-r-normal--15-*-*-*-*-*-*-*
    endif
endif

"
" Act more like a pager when invoked as one
"
if (v:progname == "view")
    nmap <Space> <PageDown>
    nmap b       <PageUp>
    nmap q       :q<CR>
endif

"
" Configure status line
"
function ModifiedFlag()
    if (&modified)
        return "*"
    else
        return " "
    endif
endfunc

""""""""""""""" Ctrl-P """""""""""""""
map <F5> :CtrlPClearAllCaches<CR>
nnoremap <leader>cc :CtrlPClearAllCaches<CR>
" Don't limit how many files to scan
let g:ctrlp_max_files = 0
let g:ctrlp_cmd = 'CtrlPCurWD'
" let g:ctrlp_mruf_exclude = '/tmp/.*\|*build*'
let g:ctrlp_custom_ignore = {
  \ 'dir':  'build$\|_build$\|node_modules$\|deps$\|coverage$\|\.git$\|\.hg$\|\.svn$\|generated-apps$\|target$',
  \ 'file': '\.class$\|\.exe$\|\.swp$\|\.pyc$\|\.so$\|\.dll$',
  \ 'link': 'some_bad_symbolic_links',
  \ }

""""""""""""""" ctags """""""""""""""
let Tlist_Ctags_Cmd='/usr/local/bin/ctags'
let Tlist_Inc_Winwidth=0
let Tlist_Show_One_File=1
let Tlist_Exist_OnlyWindow=1
let Tlist_Use_Right_Window=0
let Tlist_Sort_Type="name"
let Tlist_Display_Prototype=0
let Tlist_Compact_Format=1 " Compact?
let Tlist_GainFocus_On_ToggleOpen=1
let Tlist_Display_Tag_Scope=1
let Tlist_Close_On_Select=0
let Tlist_Enable_Fold_Column=0
let TList_WinWidth=50
map <F4> :TlistToggle<CR>
" added by jduan
" If you have multiple tag files (across different projects), or your
" current working directory changes, it is useful to have vim search
" recursively upwards for the tags file.
" set tags=tags;/
set tags+=tags;
" map Ctrl-n to finding the next tag
map <C-n> :tnext<CR>

" Along those same lines, I like for vim to change directory to
" whatever file I'm currently editing. This makes it easier to search
" for related files in the same directory - and so that :n .<Enter>
" does what's expected. It also allows the recursive tag searching
" (just mentioned above) to work properly if you edit multiple
" projects - just put the tags file at the top of each project.
" autocmd BufEnter * lcd %:p:h

" vim-ruby
syntax on             " Enable syntax highlighting
filetype on           " Enable filetype detection
filetype indent on    " Enable filetype-specific indenting
filetype plugin on    " Enable filetype-specific plugins
compiler ruby         " Enable compiler support for ruby

" load cscope
" cscope add cscope.out

" close vim when the only window left is NERDTree
autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTreeType") && b:NERDTreeType == "primary") | q | endif
" ignore node_modules/ directory
let NERDTreeIgnore=[
      \'\.beam$',
      \'\.pyc$',
      \'_build$[[dir]]',
      \'build$[[dir]]',
      \'coverage$[[dir]]',
      \'deps$[[dir]]',
      \'doc$[[dir]]',
      \'generated-apps$[[dir]]',
      \'node_modules$[[dir]]',
      \'out$[[dir]]',
      \'target$[[dir]]',
      \'bazel-.*[[dir]]',
      \]

" Press ii to exit insert mode
:imap ii <Esc>

" restore cursor position
" autocmd BufReadPost *
"     \ if line("'\"") > 1 && line("'\"") <= line("$") |
"     \   exe "normal! g`\"" |
"     \ endif

" rainbow parentheses: this plugin conflicts with VimClojure
"au VimEnter * RainbowParenthesesToggle
"au Syntax * RainbowParenthesesLoadRound
"au Syntax * RainbowParenthesesLoadSquare
"au Syntax * RainbowParenthesesLoadBraces

" vim-slime
let g:slime_target = "tmux"

" vim-javascript
let g:html_indent_inctags = "html,body,head,tbody"
let g:html_indent_script1 = "inc"
let g:html_indent_style1 = "inc"

" easy window navigation
map <C-h> <C-w>h
map <C-j> <C-w>j
" map <C-k> is used for taking notes now!
" map <C-k> <C-w>k
map <C-l> <C-w>l

" use w! save with sudo
cmap w! w !sudo tee % >/dev/null

" clear screen before running a command
map :! :!clear;

" This doesn't seem to work for Haskell files in vim 8!
"Here is an example that can be added to your .vimrc which will setup the
"supertab chaining for any filetype that has a provided |omnifunc| to
"first try that, then fall back to supertab's default, <c-p>, completion:
  " autocmd FileType *
  "   \ if &omnifunc != '' |
  "   \   call SuperTabChain(&omnifunc, "<c-p>") |
  "   \   call SuperTabSetDefaultCompletionType("<c-x><c-u>") |
  "   \ endif

" create ~/.vim_undodir if it doesn't exist yet
silent !mkdir ~/.vim_undodir > /dev/null 2>&1
" persistent undo
set undodir=~/.vim_undodir
set undofile
set undolevels=1000 "maximum number of changes that can be undone
set undoreload=10000 "maximum number lines to save for undo on a buffer reload

" make 'crontab -e' work on Mac
au FileType crontab set nobackup nowritebackup

" vimclojure
let g:vimclojure#HighlightBuiltins=1
let g:vimclojure#ParenRainbow=1
let g:vimclojure#DynamicHighlighting=1
" let vimclojure#NailgunClient = "/home/babilen/bin/ng"
" let vimclojure#WantNailgun = 1
let vimclojure#SplitPos = "right"
let vimclojure#FuzzyIndent = 1

" swap the current word with the next one without changing cursor position
:nnoremap <silent> gw "_yiw:s/\(\%#\w\+\)\(\_W\+\)\(\w\+\)/\3\2\1/<CR><c-o><c-l>
" swap the current word with the previous, keeping cursor on current word
:nnoremap <silent> gb "_yiw?\w\+\_W\+\%#<CR>:s/\(\%#\w\+\)\(\_W\+\)\(\w\+\)/\3\2\1/<CR><c-o><c-l>

" disable c-n mapping for the multichange plugin
let g:multichange_mapping = ''

" make the gutter space cleaner
highlight clear SignColumn

" syntastic general
let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0
" syntastic python
:let g:syntastic_python_checkers = ['flake8']
" syntastic coffeelint
let g:syntastic_coffee_coffeelint_args="--csv -f /Users/jingjing.duan/.coffeelint.json"
let g:syntastic_javascript_checkers = ['jshint']
" let g:syntastic_javascript_jslint_conf = "--sloppy --vars"

" turn on/off checks for given languages
" "active" means it's on
" "passive" means it's on
let g:syntastic_mode_map={ 'mode': 'active',
                     \ 'active_filetypes': [],
                     \ 'passive_filetypes': ['java', 'scala'] }

" scala format
au BufEnter *.scala setl formatprg=java\ -jar\ ~/scripts/scalariform.jar\ --stdin\ --stdout

" EasyMotion
"hi link EasyMotionTarget ErrorMsg
"hi link EasyMotionShade  Comment
hi EasyMotionTarget ctermbg=none ctermfg=red
hi EasyMotionShade  ctermbg=none ctermfg=blue

" this should be set after EasyMotion above; otherwise the color looks off!
colorscheme railscasts   " use railscasts color scheme

" This should be after "colorscheme railscast" because that scheme seems to disable these matches!
" 1. highlight trailing whitespace in red
" 2. have this highlighting not appear whilst you are typing in insert mode
" 3. have the highlighting of whitespace apply when you open new buffers
highlight ExtraWhitespace ctermbg=red guibg=red
match ExtraWhitespace /\s\+$/
autocmd BufWinEnter * match ExtraWhitespace /\s\+$/
autocmd InsertEnter * match ExtraWhitespace /\s\+\%#\@<!$/
autocmd InsertLeave * match ExtraWhitespace /\s\+$/
autocmd BufWinLeave * call clearmatches()

" vim-session
" automatically save the current session when quiting vim
:let g:session_autosave = 'yes'
" don't open the "default" or prompt for anything when starting vim
:let g:session_autoload = 'no'
" Enable aliases from SessionOpen to OpenSession and etc
:let g:session_command_aliases = 1

" format json: visually select a region and hit j
vmap <leader>j !python -m json.tool<CR>

" Avoid scrolling when switch buffers
" When switching buffers, preserve window view.
" http://vim.wikia.com/wiki/Avoid_scrolling_when_switch_buffers
if v:version >= 700
  au BufLeave * if !&diff | let b:winview = winsaveview() | endif
  au BufEnter * if exists('b:winview') && !&diff | call winrestview(b:winview) | endif
endif

" Delete a line, content only, not the line itself
noremap ]d <Esc>0D

" clever-f.vim plugin
" only search on the current line
:let g:clever_f_across_no_line = 1
" always search forward with f and backward with F
:let g:clever_f_fix_key_direction = 1

" map ; to :
noremap ; :

" dragvisuals.vim
vmap  <expr>  <LEFT>   DVB_Drag('left')
vmap  <expr>  <RIGHT>  DVB_Drag('right')
vmap  <expr>  <DOWN>   DVB_Drag('down')
vmap  <expr>  <UP>     DVB_Drag('up')
vmap  <expr>  D        DVB_Duplicate()

" javascript
au BufEnter *.js,*.coffee setl shiftwidth=4
" shell scripts
au BufEnter *.sh setl shiftwidth=2

" preserve 'no end of line'
:let g:PreserveNoEOL = 1

" Pbyank in visual mode
vnoremap Y :Pbyank<cr>

" Haskell
"
" vimproc plugin is used by ghc-mod
" To make vimproc work, you need to build its native extensions
" * cd .vim/bundle/vimproc.vim
" * make
"
" ghc-mod plugin
" You need to 'stack install ghc-mod' first.
map <silent> tw :GhcModTypeInsert<CR>
map <silent> ts :GhcModSplitFunCase<CR>
map <silent> tq :GhcModType<CR>
map <silent> te :GhcModTypeClear<CR>

" Haskell indent 2 spaces
au BufEnter *.hs setl shiftwidth=2"

" Haskell vim2hs
" disable concealing of "enumerations": commatized lists like
" deriving clauses and LANGUAGE pragmas,
" otherwise collapsed into a single ellipsis
let g:haskell_conceal_enumerations = 0

" vim-hindent format haskell file when it's saved!
" https://github.com/alx741/vim-hindent
" let g:hindent_on_save = 1
" let g:hindent_indent_size = 2 " Number of spaces per indentation
" let g:hindent_line_length = 80

" Haskell hlint
" Install "hlint" via stack and the Syntastic plugin will work out of the box.

" Build tags upon buffer save
" :function BuildHaskellTags()
" :echom  system("hasktags -c .")
" :endfunction
" uncomment this when working on large projects!
" autocmd FileType haskell autocmd BufWritePost * call BuildHaskellTags()

" https://github.com/Yggdroot/indentLine
let g:indentLine_fileTypeExclude = ['haskell']

" fzf
nnoremap <leader>f :Files<CR>

" Quit vim if quickfix is the only window visible and only tab.
aug QFClose
  au!
  au WinEnter * if winnr('$') == 1 && getbufvar(winbufnr(winnr()), "&buftype") == "quickfix"|q|endif
aug END

" Fugitive
let g:github_enterprise_urls = ['https://git.musta.ch']

" notational-fzf-vim
" Create a symlink to the real directory that has my personal notes.
let g:nv_search_paths = ['~/notes/']
noremap <C-k> :NV<CR>

" vim-markdown-preview
" let vim_markdown_preview_github=1
" let vim_markdown_preview_browser='Google Chrome'
" let vim_markdown_preview_hotkey='<C-S-m>'

" save and close a buffer
:command BWD write|bdelete

" auto make all parent dirs recurisvely if they don't exist yet
augroup Mkdir
  autocmd!
  autocmd BufWritePre * call mkdir(expand("<afile>:p:h"), "p")
augroup END

" https://vimtricks.com/p/vim-file-templates/
" file templates
autocmd BufNewFile *.sh 0r ~/.vim_skeletons/bash.sh

" Enable vim-auto-save plugin
"let g:auto_save = 1  " enable AutoSave on Vim startup

" macro that inserts the current date
nnoremap <leader>d "=strftime("%Y/%m/%d")<CR>P

" execute bash script
nnoremap <leader>r :!bash %<CR>

"""" autosave all buffers periodically """"
function! AutosaveAllBuffers(timer)
  " Check if the buffer is modified, and if so, write it
  silent! wa
endfunction

" Autosave every 1 minutes (in milliseconds)
let g:autosave_interval = 60 * 1000

" Start the timer to periodically autosave
autocmd VimEnter * let g:autosave_timer = timer_start(g:autosave_interval, 'AutosaveAllBuffers', {'repeat': -1})
"""" autosave all buffers periodically """"
