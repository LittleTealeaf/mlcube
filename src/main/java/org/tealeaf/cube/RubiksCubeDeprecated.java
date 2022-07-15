package org.tealeaf.cube;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 */
public class RubiksCubeDeprecated {

    private static final PointDeprecated[][] DISPLAY_2D = {
            {
                    null, null, null, PointDeprecated.WOG, PointDeprecated.WO, PointDeprecated.WOB, null, null, null
            }, {
                    null, null, null, PointDeprecated.WG, PointDeprecated.W, PointDeprecated.WB, null, null, null
            }, {
                    null, null, null, PointDeprecated.WRG, PointDeprecated.WR, PointDeprecated.WRB, null, null, null
            }, {
                    PointDeprecated.GWO, PointDeprecated.GW, PointDeprecated.GYR, PointDeprecated.RYG, PointDeprecated.RY, PointDeprecated.RYB, PointDeprecated.BYR, PointDeprecated.BY, PointDeprecated.BYO, PointDeprecated.OYB, PointDeprecated.OY, PointDeprecated.OYG
            }, {
                    PointDeprecated.GO, PointDeprecated.G, PointDeprecated.GR, PointDeprecated.RG, PointDeprecated.R, PointDeprecated.RB, PointDeprecated.BR, PointDeprecated.B, PointDeprecated.BO, PointDeprecated.OB, PointDeprecated.O, PointDeprecated.OG
            }, {
                    PointDeprecated.GYO, PointDeprecated.GY, PointDeprecated.GYR, PointDeprecated.RYG, PointDeprecated.RY, PointDeprecated.RYB, PointDeprecated.BYR, PointDeprecated.BY, PointDeprecated.BYO, PointDeprecated.OYB, PointDeprecated.OY, PointDeprecated.OYG
            }, {
                    null, null, null, PointDeprecated.YRG, PointDeprecated.YR, PointDeprecated.YRB, null, null, null
            }, {
                    null, null, null, PointDeprecated.YG, PointDeprecated.Y, PointDeprecated.YB, null, null, null
            }, {
                    null, null, null, PointDeprecated.YOG, PointDeprecated.YO, PointDeprecated.YOB, null, null, null
            }
    };

    private final Set<PieceDeprecated> pieceDeprecateds = PieceDeprecated.POINT_DEPRECATEDS.stream().map(PieceDeprecated::new).collect(Collectors.toSet());

    public RubiksCubeDeprecated() {

    }

    public void move(MoveDeprecated... moveDeprecateds) {
        for (MoveDeprecated moveDeprecated : moveDeprecateds) {
            pieceDeprecateds.forEach(moveDeprecated::apply);
        }
    }

    public void scramble(int count) {
        for (int i = 0; i < count; i++) {
            move(MoveDeprecated.values()[new Random().nextInt(MoveDeprecated.values().length)]);
        }
    }

    public Set<PieceDeprecated> getPieces() {
        return pieceDeprecateds;
    }

    public PointDeprecated getPiece(PointDeprecated pointDeprecated) {
        return Objects.requireNonNull(pieceDeprecateds.parallelStream().filter(pieceDeprecated -> pieceDeprecated.getPiece() == pointDeprecated).findFirst().orElse(null)).getPosition();
    }

    public Map<PointDeprecated, Character> makeColorMap() {
        Map<PointDeprecated, Character> map = new HashMap<>();

        pieceDeprecateds.forEach(pieceDeprecated -> {
            char[] pieceColors = pieceDeprecated.getPiece().getColors();
            char[] posColors = pieceDeprecated.getPosition().getColors();

            for (int i = 0; i < pieceColors.length; i++) {
                char color = pieceColors[i];
                PointDeprecated pointDeprecated = PointDeprecated.fromColors(posColors, posColors[i]);
                map.put(pointDeprecated, color);
            }
        });

        return map;
    }

    public String print() {
        Map<PointDeprecated, Character> map = makeColorMap();

        return Stream.of(DISPLAY_2D).map(row -> Stream.of(row).map(value -> map.getOrDefault(value, ' ').toString()).collect(Collectors.joining())).collect(
                Collectors.joining("\n"));
    }

    public String toString() {
        return pieceDeprecateds.toString();
    }
}
