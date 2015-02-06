(defun translate (str)
  (loop for c across str
    do (format t "~D"
         (let ((i (char-code c)))
           (cond
             ((and (>= i 97) (<= i 120)) (format nil "~A" (code-char (+ 2 i))))
             ((or (= i 121) (= i 122)) (format nil "~A" (code-char (- i 24))))
             (t " "))))))

(let ((str "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."))
 (translate str))

(format t "~%~%")

(translate "map")
