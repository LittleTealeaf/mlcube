package org.tealeaf.environment;

import java.util.HashMap;
import java.util.Map;

public enum Move {
    ;


    Move(Map<Position,Position>... permutations) {

    }

    private static Map<Position,Position> prime(Map<Position,Position> map) {
        Map<Position,Position> newMap = new HashMap<>();
        map.forEach((key,value) -> newMap.put(value,key));
        return newMap;
    }

    private static Map<Position,Position> two(Map<Position,Position> map) {
        Map<Position,Position> newMap = new HashMap<>();
        map.forEach((key,value) -> newMap.put(key,map.get(value)));
        return newMap;
    }
}
