
pub mod vector;

#[cfg(test)]
mod vector_tests {
    use super::vector::Vector;

    #[test]
    fn create_empty_vector() {
        let v = Vector::empty();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn create_fill_vector() {
        let v = Vector::fill(1.23);
        assert_eq!(v.x, 1.23);
        assert_eq!(v.y, 1.23);
        assert_eq!(v.z, 1.23);
    }

    #[test]
    fn vector_add() {
        let v = Vector::fill(3.2);
        let v2 = Vector { x: 1.0, y: 32.0, z: 2.0 };
        let res = Vector {x: 4.2, y: 35.2, z: 5.2};
        assert_eq!(v + v2, res);
    }

    #[test]
    fn vector_sub() {
        let v = Vector::fill(3.2);
        let v2 = Vector { x: 1.0, y: 32.0, z: 2.0 };
        let res = Vector {x: 2.2, y: -28.8, z: 1.2};
        assert_eq!(v - v2, res);
    }

    #[test]
    fn vector_mult() {
        let v = Vector::fill(2.0);
        let v2 = Vector { x: 1.0, y: 3.5, z: -2.0 };
        let res = Vector {x: 2.0, y: 7.0, z: -4.0};
        assert_eq!(v * v2, res);
    }

    #[test]
    fn vector_div() {
        let v = Vector::fill(3.0);
        let v2 = Vector { x: 6.0, y: -2.0, z: 8.0 };
        let res = Vector {x: 0.5, y: -1.5, z: 0.375};
        assert_eq!(v / v2, res);
    }
}
