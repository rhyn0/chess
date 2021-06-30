#include "piece.h";

using namespace std;

vector<tuple<char, char>> Piece::getMoveList(Piece *squares[8][8], char x, char y)
{
    Piece::moveList.clear();
    return Piece::moveList;
}

char Piece::getSymbol()
{
    return Piece::symbol;
}

int Piece::getValue()
{
    return Piece::value * (Piece::color == Color::White ? 1 : -1);
}

PieceType Piece::getPieceType()
{
    return Piece::pieceType;
}

int Piece::getPositionalScore(char x, char y)
{
    return 0;
}

Pawn::Pawn(Color color)
{
    Piece::color = color;
}

std::vector<std::tuple<char, char>> Pawn::getMoveList(Piece *squares[8][8], char x, char y)
{
    Piece::moveList.clear();

    if (Pawn::color == Color::White)
    {

        if (squares[x][y + 1] == nullptr)
        {
            Piece::moveList.push_back(std::tuple<char, char>(x, y + 1)); //move pawn forward one square
        }

        if (y == 1)
        {                                                                //pawn has not moved yet
            Piece::moveList.push_back(std::tuple<char, char>(x, y + 2)); //move pawn forward two squares
        }

        if ((x < 7) && (squares[x + 1][y + 1] != nullptr) && (squares[x + 1][y + 1]->color != Pawn::color))
        {
            Piece::moveList.push_back(std::tuple<char, char>(x + 1, y + 2)); //take piece on right diagonal of pawn
        }

        if ((x > 0) && (squares[x - 1][y + 1] != nullptr) && (squares[x - 1][y + 1]->color != Pawn::color))
        {
            Piece::moveList.push_back(std::tuple<char, char>(x - 1, y + 2)); //take piece on left diagonal of pawn
        }

        if (y == 4)
        {
        }
    }

    else
    {
    }
}

int Pawn::getPositionalScore(char x, char y)
{
    if (Pawn::color == Color::White)
    {
        return Pawn::scoreBoard[7 - y][x];
    }
    else
    {
        return -Pawn::scoreBoard[y][x];
    }
}