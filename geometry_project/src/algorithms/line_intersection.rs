use std::{collections::{BTreeMap, BTreeSet}, f32::consts::E};

use crate::entities::{lexicographic2d::{LexicographicLine2d, LexicographicPoint2d}, line2d::Line2D, point2d::Point2d};

use binary_search_tree::BinarySearchTree;

pub fn naive_line_intersection(lines: &Vec<Line2D>, epsilon: f32) -> Vec<Point2d> {
    if lines.len() <= 1 {
        return Vec::new();
    }

    let mut points: Vec<Point2d> = Vec::new();

    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            if let Some(p) = lines[i].intersect(&lines[j], epsilon) {
                points.push(p);
            }
        }
    }

    points
}

pub fn naive_line_intersection_with_lines(lines: &Vec<Line2D>, epsilon: f32) -> Vec<(Point2d, Line2D, Line2D)>
{
    if lines.len() <= 1 {
        return Vec::new();
    }

    let mut points: Vec<(Point2d, Line2D, Line2D)> = Vec::new();

    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            if let Some(p) = lines[i].intersect(&lines[j], epsilon) {
                points.push((p, lines[i], lines[j]));
            }
        }
    }

    points
}

#[derive(Debug, Clone, Copy)]
enum EventType
{
    Upper,
    Intersection,
    Lower
}

//wrapper around lexicographic point assuming all finite points
#[derive(Debug, Clone, Copy)]
struct Event
{
    pub point: LexicographicPoint2d,
    pub event: EventType
}

impl Event
{
    pub fn new(point: LexicographicPoint2d, event: EventType) -> Self
    {
        Self
        {
            point,
            event
        }
    }
}

impl PartialEq for Event
{
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Event
{   
}

impl PartialOrd for Event
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.point.partial_cmp(&other.point)
    }
}

//we assume at this point that all event points are finite and valid
impl Ord for Event
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.point.partial_cmp(&other.point).unwrap_or(std::cmp::Ordering::Equal)
    }
}

//wrapper around Lexicographic line, assume all lines are finite here...
//we only use the start point for ordering
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct SweepSegment(LexicographicLine2d);

impl SweepSegment
{
    pub fn new(line: Line2D) -> Self
    {
        Self(LexicographicLine2d::new_normalized(line))
    }

    pub fn event_line(event_point: LexicographicPoint2d) -> Self
    {
        Self(LexicographicLine2d::upper_lower(event_point, event_point))
    }
}

impl PartialEq for SweepSegment
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for SweepSegment
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.start().partial_cmp(&other.0.start())
    }
}

impl Eq for SweepSegment
{
}

impl Ord for SweepSegment
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal) //assume all valid numbers
    }
}

fn segments_containing_point(tree: &BinarySearchTree<SweepSegment>, point: LexicographicPoint2d, epsilon: f32) -> (Vec<SweepSegment>, Vec<SweepSegment>)
{
    let mut contains: Vec<SweepSegment> = Vec::new();
    let mut lower: Vec<SweepSegment> = Vec::new();
    let mut start_containing: bool = false;
    for segment in tree.inorder()
    {
        if segment.0.end().approx_equals(&point.0, epsilon)
        {
            start_containing = true;
            lower.push(*segment);
            continue;
        }

        if segment.0.line().intersects_point(point.0, epsilon)
        {
            start_containing = true;
            contains.push(*segment);
        }
        else 
        {
            if start_containing
            {
                break;
            }
        }
    }

    (contains, lower)
}

fn find_next_intersection_point(left_line: &SweepSegment, right_line: &SweepSegment, point: LexicographicPoint2d, event_queue: &mut BTreeMap<Event, Vec<SweepSegment>>, epsilon: f32)
{
    if let Some(p) = left_line.0.0.intersect(&right_line.0.0, epsilon)
    {
        if LexicographicPoint2d(p) >= point
        {
            return;
        }

        let event = Event::new(LexicographicPoint2d(p), EventType::Intersection);

        if event_queue.contains_key(&event)
        {
            return;
        }

        event_queue.insert(event, Vec::new());
    }
}

pub fn line_sweep_intersection(lines: &Vec<Line2D>, epsilon: f32) -> Option<Vec<Point2d>>
{
    //TODO: This is not exactly ideal... this requires us to store a vec for each key
    //even if that key is a lower point
    //todo.. may be better for the segment_status to be a ref struct wrapping the index
    //of the list
    let mut event_queue: BTreeMap<Event, Vec<SweepSegment>> = BTreeMap::new();

    for i in 0..lines.len()
    {
        if !lines[i].is_finite()
        {
            return None; //all lines must be finite
        }   

        //normalize line, guarnetee start == upper
        let segment: SweepSegment = SweepSegment::new(lines[i]);

        let upper = Event::new(segment.0.start(), EventType::Upper);
        let lower= Event::new(segment.0.end(), EventType::Lower);

        if let Some(s) = event_queue.get_mut(&upper)
        {
            s.push(segment);
        }
        else {
            event_queue.insert(upper, vec![segment]);
        }

        if !event_queue.contains_key(&lower)
        {
            event_queue.insert(lower, Vec::new());
        }
    }
    
    let mut segment_status: BinarySearchTree<SweepSegment> = BinarySearchTree::new();
    let mut intersections: Vec<Point2d> = Vec::new();

    while !event_queue.is_empty()
    {
        let next_event = event_queue.pop_first().unwrap(); //safe checked above
        let event_point = next_event.0.point;

        let upper_segments: &Vec<SweepSegment> = &next_event.1;
        let (containing_segments, lower) = segments_containing_point(&segment_status, event_point, epsilon);

        let lower_containing: Vec<&SweepSegment> = lower.iter()
        .filter(|x| containing_segments.contains(x)).collect();

        let upper_containing: Vec<SweepSegment> = upper_segments.clone().into_iter()
        .filter(|x| containing_segments.contains(x)).collect();

        let upper_containing_lower: Vec<SweepSegment> = upper_containing.clone().into_iter()
        .filter(|x| lower.contains(x)).collect();


        if upper_containing_lower.len() > 1
        {
            intersections.push(next_event.0.point.0);
        }

        for seg in lower_containing
        {
            segment_status.remove(&seg);
        }

        let upper_len = upper_containing.len();
        for seg in upper_containing.iter()        
        {
            segment_status.insert(seg.clone());
        }

        if upper_len == 0
        {
            let event_line = SweepSegment::event_line(event_point);


            let left_line = match segment_status.predecessor(&event_line) {Some(v) => v, None => {continue;}};
            let right_line = match segment_status.successor(&event_line) {Some(v) => v, None => {continue;}};

            find_next_intersection_point(left_line, right_line, event_point, &mut event_queue, epsilon);
        }
        else 
        {
            let left_most = upper_containing.iter().min().unwrap(); //upper_len != 0
            let left_neighbour = match segment_status.predecessor(left_most) {Some(v) => v, None => {continue;}};
            find_next_intersection_point(left_neighbour, left_most, event_point, &mut event_queue, epsilon);

            let right_most = upper_containing.iter().max().unwrap(); //upper_len != 0
            let right_neighbour = match segment_status.successor(right_most) {Some(v) => v, None => {continue;}};
            find_next_intersection_point(right_most, right_neighbour, event_point, &mut event_queue, epsilon);
        }

    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;


}