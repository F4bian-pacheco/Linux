
"Inicio de Vundle


set nocompatible " be iMproved, required 
filetype off " required

" set the runtime path to include Vundle and initialize
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

 "let Vundle manege Vundle, required
Plugin 'VundleVim/Vundle.vim'
"Plugin 'gmarik/Vundle.vim'

" Inicio Plugins
    "Utilidades
Plugin 'scrooloose/nerdtree'
Plugin 'vim-airline/vim-airline'
Plugin 'vim-airline/vim-airline-themes'
Plugin 'kien/ctrlp.vim'
Plugin 'easymotion/vim-easymotion'
Plugin 'sirver/ultisnips'
Plugin 'vimlab/split-term.vim'
"Plugin 'honza/vim-snippets'

    "Temas
Plugin 'joshdick/onedark.vim'
Plugin 'morhetz/gruvbox'

    "Python
Plugin 'davidhalter/jedi-vim'
"Plugin 'vim-python/python-syntax'

    "Sintaxis en general
Plugin 'jiangmiao/auto-pairs'
"Plugin 'valloric/youcompleteme'
"let g:ycm_global_ycm_extra_conf='/home/fabianlinux/.vim/bundle/youcompleteme/third_party/ycmd/cpp/ycm/.ycm_extra_conf.py'
"let g:ycm_global_ycm_extra_conf = '~/.config/nvim/global_extra_conf.py'
"let g:ycm_autoclose_preview_window_after_insertion = 1
"let g:ycm_autoclose_preview_window_after_completion = 1

"Plugin 'tomtom/tlib_vim'
"Plugin 'garbas/vim-snipmate'
"Plugin 'octol/vim-cpp-enhanced-highlight'

"Sintaxis para rust
"Plugin 'rust-lang/rust.vim'
Plugin 'neoclide/coc.nvim', 'release'

" Fin Plugins


" All of your Plugins must be added before the following line
call vundle#end()            " required
"Configuracion de PLugins

let g:airline_theme='jellybeans'

filetype plugin indent on    " required
map <C-n> :NERDTreeToggle<CR>   " Atajo para NerdTree
let NERDTreeQuitOnOpen=1

let g:NERDTreeDirArrowExpandable = '▸'
let g:NERDTreeDirArrowCollapsible = '▾'
"jedi split
let g:jedi#use_splits_not_buffers = "right"


let g:UltiSnipsExpandTrigger="<tab>"
let g:UltiSnipsJumpForwardTrigger="<c-b>"
let g:UltiSnipsJumpBackwardTrigger="<c-z>"

let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tabline#formatter = 'default'


map <C-d> :bnext<CR>
map <A-d> :bdelete<CR>

let mapleader="\ "
nmap <Leader>s <Plug>(easymotion-s2)

inoremap <expr> <cr> coc#pum#visible() ? coc#pum#confirm() : "\<CR>"

"Configuracion General

nmap <Leader>w :w<CR>
nmap <Leader>q :q<CR>

" terminal
set splitright
map <S-t> :VTerm<CR>
set title  " Muestra el nombre del archivo en la ventana de la terminal
set relativenumber  " Muestra los números de las líneas
set mouse=a  " Permite la integración del mouse (seleccionar texto, mover el cursor)

set nowrap  " No dividir la línea si es muy larga

"set cursorline  " Resalta la línea actual
"set colorcolumn=120  " Muestra la columna límite a 120 caracteres

" Indentación a 4 espacios
set tabstop=4
set shiftwidth=4
set softtabstop=2
set shiftround
set expandtab  " Insertar espacios en lugar de <Tab>s
set encoding=utf-8
syntax enable
filetype plugin indent on
set autoindent
set scrolloff=7

set hidden  " Permitir cambiar de buffers sin tener que guardarlos

set ignorecase  " Ignorar mayúsculas al hacer una búsqueda
set smartcase  " No ignorar mayúsculas si la palabra a buscar contiene mayúsculas

set clipboard=unnamed

set spelllang=en,es  " Corregir palabras usando diccionarios en inglés y español

set termguicolors  " Activa true colors en la terminal
set background=dark  " Fondo del tema: light o dark
colorscheme onedark  " Nombre del tema
" let g:gruvbox_contrast_dark = "hard"

"split navigations
nnoremap <C-J> <C-W><C-J>
nnoremap <C-K> <C-W><C-K>
nnoremap <C-L> <C-W><C-L>
nnoremap <C-H> <C-W><C-H>
