(require racket/cmdline)
(require racket/file)

(define (run filename starting-molecule)
  (letrec ([data (parse-file filename)]
           [substitutions (first data)]
           [molecule (second data)])
    (printf "Unique one-step-derivative molecules: ~s\n"
            (length (all-replacements substitutions molecule)))
    (if starting-molecule
        (printf "Steps to derive from ~s: ~s\n"
            starting-molecule
            (steps-to-reach substitutions starting-molecule molecule))
        #f)
;    (display (all-replacements substitutions molecule))
))

(define (parse-file filename)
  (letrec ([all-lines (file->lines filename)]
           [molecule (last all-lines)]
           [sub-lines (take all-lines (- (length all-lines) 2))]
           [substitutions (map parse-line sub-lines)])
    (list substitutions molecule)
))

(define (parse-line line)
  (let ([parts (string-split line " => ")])
    (cons (first parts) (second parts))
))

(define (all-replacements subs input)
  (remove-duplicates (flatten (map (lambda (s) (all-replacements-for-sub (car s) (cdr s) input)) subs)))
)

(define (all-replacements-for-sub match repl input)
  (letrec ([positions (regexp-match-positions* match input)]
           [do-sub (lambda (p) (string-append 
             (substring input 0 (car p)) repl (substring input (cdr p))))])
  (if positions (map do-sub positions) '())
))

(define (steps-to-reach substitutions molecule target)
  ; pick some approximation for the deepest we want to go, so 
  ; we don't go forever
  (letrec ([max-steps (* 2 (string-length target))])
    (for/or ([i (in-range max-steps)])
      (printf "Trying depth ~s\n" i)
      (steps-to-reach-inner substitutions molecule target (make-hash) 0 i))
))

(define (steps-to-reach-inner substitutions molecule target seen steps max-steps)
;  (printf "...Trying ~s at ~s steps\n" molecule steps)
  (cond [(equal? molecule target) steps]
        [(hash-has-key? seen molecule) #f]
        [(>= steps max-steps) #f]
        [else 
          (hash-set! seen molecule #t)
          (for/or ([sub substitutions])
            (for/or ([new-molecule 
                      (all-replacements-for-sub (car sub) (cdr sub) molecule)])
              (steps-to-reach-inner substitutions new-molecule target seen (+ 1 steps) max-steps)))
]))

(letrec ([args (current-command-line-arguments)]
         [filename (vector-ref args 0)]
         [starting-molecule (if (< (vector-length args) 2) #f
                               (vector-ref args 1))])
  (run filename starting-molecule))
