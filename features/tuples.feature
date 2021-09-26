Feature: Tuples feature

    Scenario: A tuple with w=1.0 is a point
        Given a ← tuple(4.3, -4.2, 3.1, 1.0)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1
        And a is a point

    Scenario: A tuple with w=0 is a vector
        Given a ← tuple(4.3, -4.2, 3.1, 0.0)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1
        And a is not a point

    Scenario: point() creates tuples with w=1
        Given p ← point(4.3, -4.2, 3.1)
        Then p = tuple(4.3, -4.2, 3.1, 1.0)

    Scenario: vector() creates tuples with w=0
        Given v ← vector(4.3, -4.2, 3.1)
        Then v = tuple(4.3, -4.2, 3.1, 0.0)

    Scenario: point(4.3, -4.2, 3.1) is equal to point(4.3, -4.2, 3.1)
        Given p ← point(4.3, -4.2, 3.1)
        Then p == point(4.3, -4.2, 3.1)
        And p != vector(4.3, -4.2, 3.1)

    Scenario: Adding a vector to a point
        Given a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 0)
        Then a1 + a2 = tuple(1, 1, 6, 1)

    Scenario: Adding a point to a vector
        Given a1 ← tuple(3, -2, 5, 0) AND a2 ← tuple(-2, 3, 1, 1)
        Then a1 + a2 = tuple(1, 1, 6, 0)

    Scenario: Adding two points gives an error
        Given a1 ← tuple(3, -2, 5, 0) AND a2 ← tuple(-2, 3, 1, 1)
        Then a1 + a2 = InvalidOperation

    Scenario: Subtracting a vector to a point
        Given a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 0)
        Then a1 - a2 = tuple(5, -5, 4, 1)

    Scenario: Subtracting two points must give a vector
        Given a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 1)
        Then a1 - a2 = tuple(5, -5, 4, 0)

    Scenario: Subtracting a point to a vector gives an error
        Given a1 ← tuple(3, -2, 5, 0) AND a2 ← tuple(-2, 3, 1, 1)
        Then a1 - a2 = InvalidOperation