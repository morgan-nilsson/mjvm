public class Chess {
    
    char[][] board;

    public Chess() {
        board = new char[8][8];
        this.initializeBoard();
    }

    private void initializeBoard() {
        // Initialize the chess board with pieces in their starting positions
        String[] initialSetup = {
            "rnbqkbnr",
            "pppppppp",
            "........",
            "........",
            "........",
            "........",
            "PPPPPPPP",
            "RNBQKBNR"
        };

        for (int i = 0; i < 8; i++) {
            board[i] = initialSetup[i].toCharArray();
        }
    }

}
