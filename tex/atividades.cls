\NeedsTeXFormat{LaTeX2e}

\ProvidesClass{atividades}[2026/11/10 Classe de trabalhos]

\DeclareOption*{\PassOptionsToClass{\CurrentOption}{article}}
\ProcessOptions\relax

\LoadClass[11pt]{article}

\RequirePackage[brazil]{babel}
\RequirePackage[T1]{fontenc}
\RequirePackage{lmodern}
\RequirePackage[colorlinks=true,linkcolor=red,citecolor=red,urlcolor=blue]{hyperref}
\RequirePackage{graphicx} 
\RequirePackage[top=2cm,left=2cm,right=2cm,bottom=2cm]{geometry}
\RequirePackage{amssymb,amsthm,amsmath,amsfonts}
\RequirePackage{multirow}
\RequirePackage{array}
\RequirePackage{caption}
\RequirePackage{listings}
\RequirePackage{multicol, paralist}
\RequirePackage{verbatim}
\RequirePackage{xcolor}
\RequirePackage{ifthen}
\RequirePackage[export]{adjustbox}
\RequirePackage{lastpage}
\RequirePackage[shortlabels]{enumitem} 
\RequirePackage{numprint}
\RequirePackage{minted}
\RequirePackage{tcolorbox}
\RequirePackage{etoolbox}
\RequirePackage{titling}
\RequirePackage{fp}
\RequirePackage{float}
\RequirePackage{pdflscape}
\RequirePackage{transparent}
\RequirePackage{enumitem}

\definecolor{LightGray}{RGB}{230, 230, 230}
\setminted{bgcolor=LightGray, bgcolorpadding=0.5em, style=emacs}
\nprounddigits{2}

\newcommand{\hide}[1]{%
  \makebox[0pt][l]{\textcolor{white}{#1}}%
}
\renewcommand{\baselinestretch}{1.5}

\newcommand{\setNomeAtividade}[1]{\gdef\@atividade{#1}}
\newcommand{\setCurso}[1]{\gdef\@curso{#1}}
\newcommand{\setDisciplina}[1]{\gdef\@disciplina{#1}}
\newcommand{\setSemestre}[1]{\gdef\@semestre{#1}}
\newcommand{\instituicao}{%
    {Pontifícia Universidade Católica do Paraná}%
}
\newcommand{\alert}[1]{%
    \textbf{\color{red}\ignorespaces#1}%
}

\AtBeginDocument{%
  \@ifundefined{@atividade}{%
    \ClassError{atividades}{O titulo/atividade nao foi definido}{%
      Adicione \string\setNomeAtividade{Nome} ou \string\setTitulo{Nome} no preambulo.%
    }%
  }{}%
  \@ifundefined{@curso}{%
    \ClassError{atividades}{O curso nao foi definido}{%
      Adicione \string\setCurso{Nome do Curso} no preambulo.%
    }%
  }{}%
  \@ifundefined{@disciplina}{%
    \ClassError{atividades}{A disciplina nao foi definida}{%
      Adicione \string\setDisciplina{Nome da Disciplina} no preambulo.%
    }%
  }{}%
  \@ifundefined{@semestre}{%
    \ClassError{atividades}{O semestre nao foi definido}{%
      Adicione \string\setSemestre{Ex: 2026/1} no preambulo.%
    }%
  }{}%
}

\renewcommand{\maketitle}{
    \begin{center}
       \textsc{\Large \instituicao}
    \end{center}

    \noindent \@curso \hfill \textbf{\@disciplina}

    \noindent \@semestre \hfill \theauthor 
    
    \vspace{0.5cm}
    \hrule
    \vspace{0.25cm}
    \begin{center}
        \textbf{\Large \@atividade}
    \end{center}
}%               