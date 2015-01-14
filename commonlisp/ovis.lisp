(defun get-process-stream (program args)
    (process-output (run-program program args :output :stream)))

(defun get-active-window ()
  (read-line
    (get-process-stream "/usr/bin/xprop" '("-root" "_NET_ACTIVE_WINDOW"))))

(defun get-cur-window ()
  (subseq (get-active-window) 40 49))

(defun get-char ()
  (coerce (get-cur-window) 'character))

(defun get-window-name ()
  (read-line
    (get-process-stream "/usr/bin/xprop" '("-id" (format "~s" (get-char)) "_NET_WM_NAME"))))

(print (type-of (get-cur-window)))
(print (type-of "hello"))
(print (get-cur-window))
(print (get-window-name))
