package org.tealeaf.solver;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.RepeatedTest;
import org.tealeaf.cube.Point;
import org.tealeaf.cube.RubiksCube;

import static org.junit.jupiter.api.Assertions.*;

class SolverTest {
    Solver solver;
    RubiksCube rubiksCube;

    @BeforeEach
    void setup() {
        rubiksCube = new RubiksCube();
        rubiksCube.scramble(1000);
        solver = new Solver(rubiksCube);
    }
    @RepeatedTest(50)
    void testOrient() {
        solver.orient();
        for(Point point : new Point[] {Point.W, Point.R, Point.Y, Point.G, Point.B, Point.O}) {
            assertEquals(point,rubiksCube.getPiece(point));
        }
    }

    @RepeatedTest(1000)
    void testWhiteCross() {
        solver.orient();
        solver.whiteCross();
    }
}