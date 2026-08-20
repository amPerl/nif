
use super::NiTriBasedGeomData;
use crate::common::Triangle;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTriStripsData {
    #[bw(args(base.num_triangles))]
    pub base: NiTriBasedGeomData,

    #[br(temp)]
    #[bw(calc = strip_lengths.len() as u16)]
    num_strips: u16,
    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(temp)]
    #[bw(calc = u8::from(points.is_some()))]
    has_points: u8,
    #[br(if(has_points != 0), count = strip_lengths.iter().map(|l| *l as usize).sum::<usize>())]
    pub points: Option<Vec<u16>>,
}

impl NiTriStripsData {
    pub fn strips(&self) -> impl Iterator<Item = &[u16]> {
        let points = self.points.as_deref().unwrap_or(&[]);
        let mut start = 0usize;
        self.strip_lengths.iter().map(move |length| {
            let end = start.saturating_add(*length as usize).min(points.len());
            let strip = points.get(start..end).unwrap_or(&[]);
            start = end;
            strip
        })
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.strips().flat_map(strip_triangles)
    }
}

impl std::ops::Deref for NiTriStripsData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

fn strip_triangles(strip: &[u16]) -> impl Iterator<Item = Triangle> + '_ {
    strip.windows(3).enumerate().filter_map(|(i, window)| {
        let &[p, q, r] = window else { return None };
        let (a, b, c) = if i % 2 == 0 { (p, q, r) } else { (p, r, q) };
        (a != b && b != c && c != a).then_some(Triangle { a, b, c })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indices(strip: &[u16]) -> Vec<(u16, u16, u16)> {
        strip_triangles(strip).map(|t| (t.a, t.b, t.c)).collect()
    }

    #[test]
    fn winding_alternates() {
        assert_eq!(
            indices(&[0, 1, 2, 3, 4, 5]),
            [(0, 1, 2), (1, 3, 2), (2, 3, 4), (3, 5, 4)]
        );
    }

    #[test]
    fn degenerate_triangles_are_dropped() {
        assert_eq!(
            indices(&[0, 1, 2, 2, 3, 4, 5]),
            [(0, 1, 2), (2, 4, 3), (3, 4, 5)]
        );
    }

    #[test]
    fn short_strips_yield_nothing() {
        assert!(indices(&[]).is_empty());
        assert!(indices(&[0, 1]).is_empty());
    }
}
