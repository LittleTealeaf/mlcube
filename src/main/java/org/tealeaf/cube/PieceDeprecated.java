package org.tealeaf.cube;

import java.util.Set;

/**
 */
public class PieceDeprecated {

    public static final Set<PointDeprecated> POINT_DEPRECATEDS = Set.of(PointDeprecated.W, PointDeprecated.WR, PointDeprecated.WB, PointDeprecated.WG, PointDeprecated.WO, PointDeprecated.WRB,
                                                                        PointDeprecated.WRG, PointDeprecated.WOG, PointDeprecated.WOB, PointDeprecated.Y, PointDeprecated.YR
            , PointDeprecated.YB, PointDeprecated.YG,
                                                                        PointDeprecated.YO, PointDeprecated.YRB, PointDeprecated.YRG, PointDeprecated.YOG, PointDeprecated.YOB, PointDeprecated.R, PointDeprecated.O, PointDeprecated.B, PointDeprecated.G, PointDeprecated.RB,
                                                                        PointDeprecated.RG, PointDeprecated.OB, PointDeprecated.OG);

    private final PointDeprecated piece;
    private PointDeprecated position;

    public PieceDeprecated(PointDeprecated pointDeprecated) {
        this.piece = pointDeprecated;
        this.position = pointDeprecated;
    }

    public PointDeprecated getPiece() {
        return piece;
    }

    public PointDeprecated getPosition() {
        return position;
    }

    public void setPosition(PointDeprecated position) {
        this.position = position;
    }

    public String toString() {
        return piece + "=" + position;
    }
}
