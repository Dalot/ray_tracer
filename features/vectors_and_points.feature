Feature: Vector and Points math feature

    Scenario: A tuple with w=1.0 is a point
        Given a ← point(4.3, -4.2, 3.1)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1

    Scenario: A tuple with w=0 is a vector
        Given a ← vector(4.3, -4.2, 3.1)
        Then a.x = 4.3
        And a.y = -4.2
        And a.z = 3.1

    Scenario: point() creates tuples with w=1
        Given p ← point(4.3, -4.2, 3.1)
        Then p = point(4.3, -4.2, 3.1)

    Scenario: vector() creates tuples with w=0
        Given v ← vector(4.3, -4.2, 3.1)
        Then v = vector(4.3, -4.2, 3.1)

    Scenario: point(4.3, -4.2, 3.1) is equal to point(4.3, -4.2, 3.1)
        Given p ← point(4.3, -4.2, 3.1)
        Then p == point(4.3, -4.2, 3.1)

    Scenario: Adding a vector to a point
        Given a1 ← point(3, -2, 5) AND a2 ← vector(-2, 3, 1)
        Then a1 + a2 = tuple(1, 1, 6, 1)

    Scenario: Adding a point to a vector
        Given a1 ← vector(3, -2, 5) AND a2 ← point(-2, 3, 1)
        Then a1 + a2 = tuple(1, 1, 6, 0)

    Scenario: Subtracting a vector to a point
        Given a1 ← point(3, -2, 5) AND a2 ← vector(-2, 3, 1)
        Then a1 - a2 = point(5, -5, 4)

    Scenario: Subtracting two points must give a vector
        Given a1 ← point(3, -2, 5) AND a2 ← point(-2, 3, 1)
        Then a1 - a2 = vector(5, -5, 4)

    Scenario: Negate a vector
        Given v ← vector(1, -2, 3)
        Then -v = vector(-1, 2, -3)

    Scenario: Multiplying a tuple by a scalar
        Given a ← vector(1, -2, 3)
        Then a * 3.5 = vector(3.5, -7, 10.5)

    Scenario: Multiplying a tuple by a fraction
        Given a ← vector(1, -2, 3)
        Then a * 0.5 = vector(0.5, -1, 1.5)

    Scenario: Dividing a tuple by a scalar
        Given a ← vector(1, -2, 3)
        Then a / 2 = vector(0.5, -1, 1.5)

    Scenario: Computing the magnitude of vector(1, 0, 0)
        Given v ← vector(1, 0, 0)
        Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(0, 1, 0)
        Given v ← vector(0, 1, 0)
        Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(0, 0, 1)
        Given v ← vector(0, 0, 1)
        Then magnitude(v) = 1

    Scenario: Computing the magnitude of vector(1, 2, 3)
        Given [1] v ← vector(1, 2, 3)
        Then magnitude(v) = √14

    Scenario: Computing the magnitude of vector(-1, -2, -3)
        Given [2] v ← vector(-1, -2, -3)
        Then magnitude(v) = √14

    Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
        Given v ← vector(4, 0, 0)
        Then normalize(v) = vector(1, 0, 0)

    Scenario: Normalizing vector(1, 2, 3)
        Given [normalization] v ← vector(1, 2, 3)
        Then normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)

    Scenario: The magnitude of a normalized vector
        Given [3] v ← vector(1, 2, 3)
        When norm ← normalize(v)
        Then magnitude(norm) = 1

    Scenario: The dot product of two tuples
        Given a ← vector(1, 2, 3) AND b ← vector(2, 3, 4)
        Then dot(a, b) = 20

    Scenario: The cross product of two vectors
        Given [2] a ← vector(1, 2, 3) And b ← vector(2, 3, 4)
        Then cross(a, b) = vector(-1, 2, -1)
        And cross(b, a) = vector(1, -2, 1)



