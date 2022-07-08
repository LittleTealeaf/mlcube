package org.tealeaf.cube;

import java.util.Set;

public class Piece {

    public static final Set<Point> POINTS = Set.of(Point.W, Point.WR, Point.WB, Point.WG, Point.WO, Point.WRB,
            Point.WRG, Point.WOG, Point.WOB, Point.Y, Point.YR, Point.YB, Point.YG,
            Point.YO, Point.YRB, Point.YRG, Point.YOG, Point.YOB, Point.R, Point.O, Point.B, Point.G, Point.RB,
            Point.RG, Point.OB, Point.OG);

    private final Point piece;
    private Point position;

    public Piece(Point point) {
        this.piece = point;
        this.position = point;
    }

    public Point getPiece() {
        return piece;
    }

    public Point getPosition() {
        return position;
    }

    public void setPosition(Point position) {
        this.position = position;
    }

    public String toString() {
        return piece + "=" + position;
    }
}
