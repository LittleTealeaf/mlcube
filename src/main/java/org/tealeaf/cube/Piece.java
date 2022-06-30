package org.tealeaf.cube;

public class Piece {



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
}
