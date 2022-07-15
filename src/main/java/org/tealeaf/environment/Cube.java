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
                Position.W,
                Position.WB,
                Position.WR,
                Position.WO,
                Position.WG,
                Position.WBR,
                Position.WBO,
                Position.WGR,
                Position.WGO,
                Position.Y,
                Position.YG,
                Position.YR,
                Position.YB,
                Position.YO,
                Position.YGR,
                Position.YGO,
                Position.YBR,
                Position.YBO,
                Position.G,
                Position.GR,
                Position.GO,
                Position.O,
                Position.R,
                Position.B,
                Position.BR,
                Position.BO
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
