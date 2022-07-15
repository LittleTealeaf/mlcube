package org.tealeaf.environment;

import java.util.HashMap;
import java.util.Map;

public class Cube {

    private Map<Position, Position> pieces;

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
        pieces = new HashMap<>(piece_locations.length);
        for (Position piece_location : piece_locations) {
            pieces.put(piece_location,piece_location);
        }
    }

    @Override
    public String toString() {
        return pieces.toString();
    }
}
