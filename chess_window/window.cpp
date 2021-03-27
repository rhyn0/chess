#include <QApplication>
#include <QPushButton>

#include "window.h"

Window::Window(QWidget *parent) :
 QWidget(parent)
 {
  // Set size of the window
  setFixedSize(1000, 500);

  // Create and position the button
  m_button = new QPushButton("Hello World", this);
  m_button->setGeometry(450, 200, 100, 100);

  // NEW : Do the connection
  connect(m_button, SIGNAL (clicked()), QApplication::instance(), SLOT (quit()));
 }
