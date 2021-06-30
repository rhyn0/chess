#include <tuple>
#include <vector>

enum class Color
{
    None,
    White,
    Black
};
enum class PieceType
{
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
};

class Piece
{
private:
    static const char symbol = ' ';
    static const int value = 0;
    static const PieceType pieceType = PieceType::None;

public:
    Color color;
    virtual char getSymbol();
    virtual int getValue();
    virtual PieceType getPieceType();
    virtual int getPositionalScore(char x, char y);
    std::vector<std::tuple<char, char>> moveList;
    virtual std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y);
};

class Pawn : public Piece
{
private:
    static const char symbol = 'p';
    static const int value = 1;
    static const PieceType pieceType = PieceType::Pawn;

public:
    Pawn(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};

class Knight : public Piece
{
private:
    static const char symbol = 'n';
    static const int value = 3;
    static const PieceType pieceType = PieceType::Knight;

public:
    Knight(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};

class Bishop : public Piece
{
private:
    static const char symbol = 'b';
    static const int value = 3;
    static const PieceType pieceType = PieceType::Bishop;

public:
    Bishop(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};

class Rook : public Piece
{
private:
    static const char symbol = 'r';
    static const int value = 5;
    static const PieceType pieceType = PieceType::Rook;

public:
    Rook(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};

class Queen : public Piece
{
private:
    static const char symbol = 'q';
    static const int value = 9;
    static const PieceType pieceType = PieceType::Queen;

public:
    Queen(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};

class King : public Piece
{
private:
    static const char symbol = 'k';
    static const int value = 900;
    static const PieceType pieceType = PieceType::King;

public:
    King(Color color);
    // char getSymbol() override;
    // int getValue() override;
    // PieceType getPieceType() override;
    int getPositionalScore(char x, char y) override;
    std::vector<std::tuple<char, char>> getMoveList(Piece *squares[8][8], char x, char y) override;
};