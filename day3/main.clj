(let [content (slurp "input.txt")
      re #"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)"]
  (reduce
   (fn [{:keys [enabled sum cond-sum]} cmd]
     (cond
       (= (first cmd) "do()") {:enabled true :sum sum :cond-sum cond-sum}
       (= (first cmd) "don't()") {:enabled false :sum sum :cond-sum cond-sum}
       :else (let [[_ x y] cmd
                   x (Integer. x)
                   y (Integer. y)
                   product (* x y)]
               {:enabled enabled :sum (+ sum product) :cond-sum (if enabled (+ cond-sum product) cond-sum)})))
   {:enabled true :sum 0 :cond-sum 0}
   (re-seq re content)))
