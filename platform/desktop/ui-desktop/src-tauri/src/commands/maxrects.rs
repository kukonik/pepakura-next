#[derive(Clone, Copy)]
pub struct Rect { pub x: f64, pub y: f64, pub w: f64, pub h: f64 }

pub struct MaxRectsPacker { bins: Vec<Rect>, pub width: f64, pub height: f64 }

impl MaxRectsPacker {
    pub fn new(width: f64, height: f64) -> Self {
        Self { bins: vec![Rect { x: 0.0, y: 0.0, w: width, h: height }], width, height }
    }

    fn overlaps(a: &Rect, b: &Rect) -> bool {
        !(a.x >= b.x + b.w || a.x + a.w <= b.x || a.y >= b.y + b.h || a.y + a.h <= b.y)
    }

    pub fn insert(&mut self, w: f64, h: f64) -> Option<(f64, f64)> {
        if w <= 0.0 || h <= 0.0 { return None; }
        let mut best_score1 = f64::MAX;
        let mut best_score2 = f64::MAX;
        let mut best_idx = None;
        let mut best_x = 0.0;
        let mut best_y = 0.0;

        for (i, bin) in self.bins.iter().enumerate() {
            if bin.w < w || bin.h < h { continue; }
            let short_side_fit = (bin.w - w).abs().min((bin.h - h).abs());
            let long_side_fit = (bin.w - w).abs().max((bin.h - h).abs());
            if short_side_fit < best_score1 || (short_side_fit == best_score1 && long_side_fit < best_score2) {
                best_score1 = short_side_fit; best_score2 = long_side_fit;
                best_idx = Some(i); best_x = bin.x; best_y = bin.y;
            }
        }

        if let Some(idx) = best_idx {
            let placed = Rect { x: best_x, y: best_y, w, h };
            let mut new_bins = Vec::new();
            if best_x + w < self.bins[idx].x + self.bins[idx].w {
                new_bins.push(Rect { x: best_x + w, y: self.bins[idx].y, w: self.bins[idx].x + self.bins[idx].w - best_x - w, h: self.bins[idx].h });
            }
            if best_y + h < self.bins[idx].y + self.bins[idx].h {
                new_bins.push(Rect { x: self.bins[idx].x, y: best_y + h, w: self.bins[idx].w, h: self.bins[idx].y + self.bins[idx].h - best_y - h });
            }
            let mut final_bins = Vec::new();
            for b in self.bins.iter().chain(new_bins.iter()) {
                if !Self::overlaps(&placed, b) { final_bins.push(*b); }
            }
            self.bins = final_bins;
            Some((best_x, best_y))
        } else { None }
    }
}
