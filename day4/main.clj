(require '[clojure.string :as str])

(let [content (slurp "input.txt")
      lines (str/split-lines content)
      width (count (nth lines 0))
      height (count lines)
      char-getter (fn [x y]
                    (nth
                     (nth lines y '())
                     x \z))
      word-getter (fn [x y dx dy]
                    (apply str
                           (map
                            #(char-getter (+ x (* % dx)) (+ y (* % dy)))
                            (range 4))))
      words-getter (fn [x y]
                      (map
                       (fn [dx]
                         (map
                          (fn [dy] (word-getter x y dx dy))
                          (range -1 2)))
                       (range -1 2)))]
  (count
   (filter
    #(= "XMAS" %)
    (flatten
     (map
      (fn [y]
        (map
         (fn [x] (words-getter x y))
         (range width)))
      (range height))))))
