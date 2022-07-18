package org.tealeaf.environment;

import java.util.Map;

public class Piece {

    private final Position piece;
    private Position position;

    public Piece(Position position) {
        this.piece = position;
        this.position = position;
    }

    public Piece(Position piece, Position position) {
        this.piece = piece;
        this.position = position;
    }

    public Position getPiece() {
        return piece;
    }

    public Position getPosition() {
        return position;
    }

    public void setPosition(Position position) {
        this.position = position;
    }

    @Override
    public String toString() {
        return String.format("%s:%s", piece.toString(),position.toString());
    }
}
