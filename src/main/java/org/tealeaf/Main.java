package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.RubiksCube;

public class Main {

    public static void main(String[] args) {
        RubiksCube rubiksCube = new RubiksCube();
        System.out.println(rubiksCube.print2d());
        rubiksCube.move(Move.yP);
        System.out.println(rubiksCube.print2d());

    }


}
