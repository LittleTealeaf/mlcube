package org.tealeaf.environment;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Cube {

    private final Set<Piece> pieces;

    public Cube() {
        Position[] piece_locations = {
                Position.B,
                Position.BO,
                Position.BR,
                Position.G,
                Position.GO,
                Position.GR,
                Position.O,
                Position.R,
                Position.W,
                Position.WB,
                Position.WBO,
                Position.WBR,
                Position.WG,
                Position.WGO,
                Position.WGR,
                Position.WO,
                Position.WR,
                Position.Y,
                Position.YB,
                Position.YBO,
                Position.YBR,
                Position.YG,
                Position.YGO,
                Position.YGR,
                Position.YO,
                Position.YR
        };
        pieces = Stream.of(piece_locations).map(Piece::new).collect(Collectors.toSet());
    }

    public void move(Move move) {
        pieces.forEach(move::apply);
    }

    @Override
    public String toString() {
        return pieces.toString();
    }
}
