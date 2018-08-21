(defn f [x]
  (let [y "world"]
    (if (= y "e")
      (println "error")
      (println x " " y "!"))))

(defn main []
  (f "hello"))
