TEMPLATE = app
TARGET = chess_window.pro

QT = core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

SOURCES += \
    main_window.cpp \
    window.cpp

HEADERS += \
    window.h
