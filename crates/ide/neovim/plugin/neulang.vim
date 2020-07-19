if exists('g:loaded_neulang')
    finish
endif

let g:loaded_neulang=1

if !exists('s:jobid')
	let s:jobid = 0
endif


let s:scriptdir = resolve(expand('<sfile>:p:h') . '/../../../../')
if !exists('g:neulang_program')
    let g:neulang_program = s:scriptdir . '/target/release/neu-nvim'
endif

function! neulang#init()
    call neulang#connect()
endfunction

function! neulang#connect()
  let result = s:StartJob()

  if 0 == result
    echoerr "neulang: cannot start rpc process"
  elseif -1 == result
    echoerr "neulang: rpc process is not executable"
  else
    let s:jobid = result
    call s:ConfigureJob(result)
  endif
endfunction

function! neulang#reset()
    let s:jobid = 0
endfunction

function! s:ConfigureJob(jobid)
    augroup neulang
        autocmd!

        autocmd BufReadPre,FileReadPre *.md :call s:Load() " *.neu
    augroup END
endfunction

let s:MsgLoad = 'load'

function! s:Load(...)
    call rpcnotify(s:jobid, s:MsgLoad)
endfunction

function! s:OnStderr(id, data, event) dict
  echom 'neulang: stderr: ' . join(a:data, "\n")
endfunction

function! s:StartJob()
    if 0 == s:jobid
        let id = jobstart([g:neulang_program], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
        return id
    else
        return 0
    endif
endfunction

function! s:StopJob()
  if 0 < s:jobid
    augroup neulang
      autocmd!
    augroup END

    call rpcnotify(s:jobid, 'quit')
    let result = jobwait(s:jobid, 500)

    if -1 == result
      call jobstop(s:jobid)
    endif

    let s:jobid = 0
  endif
endfunction

call neulang#connect()
