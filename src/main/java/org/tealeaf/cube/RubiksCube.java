package org.tealeaf.cube;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class RubiksCube {

    private static final Point[][] DISPLAY_2D = {
            {
                    null, null, null, Point.WOG, Point.WO, Point.WOB, null, null, null
            }, {
                    null, null, null, Point.WG, Point.W, Point.WB, null, null, null
            }, {
                    null, null, null, Point.WRG, Point.WR, Point.WRB, null, null, null
            }, {
                    Point.GWO, Point.GW, Point.GYR, Point.RYG, Point.RY, Point.RYB, Point.BYR, Point.BY, Point.BYO, Point.OYB, Point.OY, Point.OYG
            }, {
                    Point.GO, Point.G, Point.GR, Point.RG, Point.R, Point.RB, Point.BR, Point.B, Point.BO, Point.OB, Point.O, Point.OG
            }, {
                    Point.GYO, Point.GY, Point.GYR, Point.RYG, Point.RY, Point.RYB, Point.BYR, Point.BY, Point.BYO, Point.OYB, Point.OY, Point.OYG
            }, {
                    null, null, null, Point.YRG, Point.YR, Point.YRB, null, null, null
            }, {
                    null, null, null, Point.YG, Point.Y, Point.YB, null, null, null
            }, {
                    null, null, null, Point.YOG, Point.YO, Point.YOB, null, null, null
            }
    };

    private final Set<Piece> pieces = Piece.POINTS.stream().map(Piece::new).collect(Collectors.toSet());

    public RubiksCube() {

    }

    public void move(Move... moves) {
        for (Move move : moves) {
            pieces.forEach(move::apply);
        }
    }

    public void scramble(int count) {
        for (int i = 0; i < count; i++) {
            move(Move.values()[new Random().nextInt(Move.values().length)]);
        }
    }

    public Set<Piece> getPieces() {
        return pieces;
    }

    public Point getPiece(Point point) {
        return Objects.requireNonNull(pieces.parallelStream().filter(piece -> piece.getPiece() == point).findFirst().orElse(null)).getPosition();
    }

    public Map<Point, Character> makeColorMap() {
        Map<Point, Character> map = new HashMap<>();

        pieces.forEach(piece -> {
            char[] pieceColors = piece.getPiece().getColors();
            char[] posColors = piece.getPosition().getColors();

            for (int i = 0; i < pieceColors.length; i++) {
                char color = pieceColors[i];
                Point point = Point.fromColors(posColors, posColors[i]);
                map.put(point, color);
            }
        });

        return map;
    }

    public String print2d() {
        Map<Point, Character> map = makeColorMap();

        return Stream.of(DISPLAY_2D).map(row -> Stream.of(row).map(value -> map.getOrDefault(value, ' ').toString()).collect(Collectors.joining())).collect(
                Collectors.joining("\n"));
    }

    public String toString() {
        return pieces.toString();
    }
}
