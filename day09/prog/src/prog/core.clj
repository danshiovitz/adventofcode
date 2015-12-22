(ns prog.core
  (:gen-class)
  (:require [clojure.math.combinatorics :as combo]))

(defn parse [line]
  (let [[_ cfrom cto dist] (re-matches #"(\S+) to (\S+) = (\d+)" line)]
     (if dist [#{cfrom cto} (Integer/parseInt dist)] nil)))

(defn load-dist-file [file]
  (into (hash-map) 
    (with-open [rdr (clojure.java.io/reader file)] 
      (doall (map parse (line-seq rdr))))))

(defn score [t dist-map]
  (reduce + (map dist-map (map set (partition 2 1 t)))))

(defn -main
  [& args]
  (let [dist-map (load-dist-file (first args))
        places (set (mapcat identity (keys dist-map)))
        traversals (combo/permutations places)
        with-score (fn [t] [t (score t dist-map)])
        scored-traversals (map with-score traversals)
        best-traversal (apply max-key second scored-traversals)]
    (println best-traversal)))
