package org.tealeaf.cube;

import java.util.Arrays;

public enum Point {
    B,
    BO,
    BR,
    BW,
    BWO,
    BWR,
    BY,
    BYO,
    BYR,
    G,
    GO,
    GR,
    GW,
    GWO,
    GWR,
    GY,
    GYO,
    GYR,
    O,
    OB,
    OG,
    OW,
    OWB,
    OWG,
    OY,
    OYB,
    OYG,
    R,
    RB,
    RG,
    RW,
    RWB,
    RWG,
    RY,
    RYB,
    RYG,
    W,
    WB,
    WG,
    WO,
    WOB,
    WOG,
    WR,
    WRB,
    WRG,
    Y,
    YB,
    YG,
    YO,
    YOB,
    YOG,
    YR,
    YRB,
    YRG;

    public static final char[] COLOR_ORDER = "WYROBG".toCharArray();

    private static final char NULL = '\u0000';

    public static Point fromColors(char[] colors) {
        return valueOf(new String(colors));
    }

    public static Point fromColors(char[] colors, char first) {
        char[] pool = Arrays.copyOf(colors, colors.length);
        Arrays.sort(pool);

        char[] out = new char[colors.length];

        // put the first character at the beginning
        out[0] = first;

        // null out the first character
        pool[Arrays.binarySearch(pool, first)] = NULL;

        // cycle through the next ones
        for (int i = 1; i < out.length; i++) {
            char a = NULL;
            int index = 0;
            for (char c : pool) {
                if (c != NULL) {
                    for (int value = 0; value < COLOR_ORDER.length; value++) {
                        if (COLOR_ORDER[value] == c) {
                            if (a == NULL || value < index) {
                                a = c;
                                index = value;
                            }
                            break;
                        }
                    }
                }
            }
            for (int b = 0; b < pool.length; b++) {
                if (pool[b] == a) {
                    pool[b] = NULL;
                }
            }

            out[i] = a;
        }

        return valueOf(new String(out));
    }

    public char[] getColors() {
        return toString().toCharArray();
    }
}
