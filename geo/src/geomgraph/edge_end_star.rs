// JTS: import java.io.PrintStream;
// JTS: import java.util.ArrayList;
// JTS: import java.util.Iterator;
// JTS: import java.util.List;
// JTS: import java.util.Map;
// JTS: import java.util.TreeMap;
// JTS:
// JTS: import org.locationtech.jts.algorithm.BoundaryNodeRule;
// JTS: import org.locationtech.jts.algorithm.locate.SimplePointInAreaLocator;
// JTS: import org.locationtech.jts.geom.Coordinate;
// JTS: import org.locationtech.jts.geom.Location;
// JTS: import org.locationtech.jts.geom.TopologyException;
// JTS: import org.locationtech.jts.util.Assert;
// JTS:
// JTS: /**
// JTS:  * A EdgeEndStar is an ordered list of EdgeEnds around a node.
// JTS:  * They are maintained in CCW order (starting with the positive x-axis) around the node
// JTS:  * for efficient lookup and topology building.
// JTS:  *
// JTS:  * @version 1.7
// JTS:  */
// JTS: abstract public class EdgeEndStar
// TODO: Or maybe trait?
#[derive(Clone)]
pub struct EdgeEndStar;
// JTS: {
// JTS:
// JTS:   /**
// JTS:    * A map which maintains the edges in sorted order around the node
// JTS:    */
// JTS:   protected Map edgeMap = new TreeMap();
// JTS:   /**
// JTS:    * A list of all outgoing edges in the result, in CCW order
// JTS:    */
// JTS:   protected List edgeList;
// JTS:   /**
// JTS:    * The location of the point for this star in Geometry i Areas
// JTS:    */
// JTS:   private int[] ptInAreaLocation = { Location.NONE, Location.NONE };
// JTS:
// JTS:   public EdgeEndStar()
// JTS:   {
// JTS:
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * Insert a EdgeEnd into this EdgeEndStar
// JTS:    */
// JTS:   abstract public void insert(EdgeEnd e);
// JTS:
// JTS:   /**
// JTS:    * Insert an EdgeEnd into the map, and clear the edgeList cache,
// JTS:    * since the list of edges has now changed
// JTS:    */
// JTS:   protected void insertEdgeEnd(EdgeEnd e, Object obj)
// JTS:   {
// JTS:     edgeMap.put(e, obj);
// JTS:     edgeList = null;  // edge list has changed - clear the cache
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * @return the coordinate for the node this star is based at
// JTS:    */
// JTS:   public Coordinate getCoordinate()
// JTS:   {
// JTS:     Iterator it = iterator();
// JTS:     if (! it.hasNext()) return null;
// JTS:     EdgeEnd e = (EdgeEnd) it.next();
// JTS:     return e.getCoordinate();
// JTS:   }
// JTS:   public int getDegree()
// JTS:   {
// JTS:     return edgeMap.size();
// JTS:   }
// JTS:
// JTS:   /**
// JTS:    * Iterator access to the ordered list of edges is optimized by
// JTS:    * copying the map collection to a list.  (This assumes that
// JTS:    * once an iterator is requested, it is likely that insertion into
// JTS:    * the map is complete).
// JTS:    */
// JTS:   public Iterator iterator()
// JTS:   {
// JTS:     return getEdges().iterator();
// JTS:   }
// JTS:   public List getEdges()
// JTS:   {
// JTS:     if (edgeList == null) {
// JTS:       edgeList = new ArrayList(edgeMap.values());
// JTS:     }
// JTS:     return edgeList;
// JTS:   }
// JTS:   public EdgeEnd getNextCW(EdgeEnd ee)
// JTS:   {
// JTS:     getEdges();
// JTS:     int i = edgeList.indexOf(ee);
// JTS:     int iNextCW = i - 1;
// JTS:     if (i == 0)
// JTS:       iNextCW = edgeList.size() - 1;
// JTS:     return (EdgeEnd) edgeList.get(iNextCW);
// JTS:   }
// JTS:
// JTS:   public void computeLabelling(GeometryGraph[] geomGraph)
// JTS:   {
// JTS:     computeEdgeEndLabels(geomGraph[0].getBoundaryNodeRule());
// JTS:     // Propagate side labels  around the edges in the star
// JTS:     // for each parent Geometry
// JTS: //Debug.print(this);
// JTS:     propagateSideLabels(0);
// JTS: //Debug.print(this);
// JTS: //Debug.printIfWatch(this);
// JTS:     propagateSideLabels(1);
// JTS: //Debug.print(this);
// JTS: //Debug.printIfWatch(this);
// JTS:
// JTS:     /**
// JTS:      * If there are edges that still have null labels for a geometry
// JTS:      * this must be because there are no area edges for that geometry incident on this node.
// JTS:      * In this case, to label the edge for that geometry we must test whether the
// JTS:      * edge is in the interior of the geometry.
// JTS:      * To do this it suffices to determine whether the node for the edge is in the interior of an area.
// JTS:      * If so, the edge has location INTERIOR for the geometry.
// JTS:      * In all other cases (e.g. the node is on a line, on a point, or not on the geometry at all) the edge
// JTS:      * has the location EXTERIOR for the geometry.
// JTS:      * <p>
// JTS:      * Note that the edge cannot be on the BOUNDARY of the geometry, since then
// JTS:      * there would have been a parallel edge from the Geometry at this node also labelled BOUNDARY
// JTS:      * and this edge would have been labelled in the previous step.
// JTS:      * <p>
// JTS:      * This code causes a problem when dimensional collapses are present, since it may try and
// JTS:      * determine the location of a node where a dimensional collapse has occurred.
// JTS:      * The point should be considered to be on the EXTERIOR
// JTS:      * of the polygon, but locate() will return INTERIOR, since it is passed
// JTS:      * the original Geometry, not the collapsed version.
// JTS:      *
// JTS:      * If there are incident edges which are Line edges labelled BOUNDARY,
// JTS:      * then they must be edges resulting from dimensional collapses.
// JTS:      * In this case the other edges can be labelled EXTERIOR for this Geometry.
// JTS:      *
// JTS:      * MD 8/11/01 - NOT TRUE!  The collapsed edges may in fact be in the interior of the Geometry,
// JTS:      * which means the other edges should be labelled INTERIOR for this Geometry.
// JTS:      * Not sure how solve this...  Possibly labelling needs to be split into several phases:
// JTS:      * area label propagation, symLabel merging, then finally null label resolution.
// JTS:      */
// JTS:     boolean[] hasDimensionalCollapseEdge = { false, false };
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS:       for (int geomi = 0; geomi < 2; geomi++) {
// JTS:         if (label.isLine(geomi) && label.getLocation(geomi) == Location.BOUNDARY)
// JTS:           hasDimensionalCollapseEdge[geomi] = true;
// JTS:       }
// JTS:     }
// JTS: //Debug.print(this);
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS: //Debug.println(e);
// JTS:       for (int geomi = 0; geomi < 2; geomi++) {
// JTS:         if (label.isAnyNull(geomi)) {
// JTS:           int loc = Location.NONE;
// JTS:           if (hasDimensionalCollapseEdge[geomi]) {
// JTS:             loc = Location.EXTERIOR;
// JTS:           }
// JTS:           else {
// JTS:             Coordinate p = e.getCoordinate();
// JTS:             loc = getLocation(geomi, p, geomGraph);
// JTS:           }
// JTS:           label.setAllLocationsIfNull(geomi, loc);
// JTS:         }
// JTS:       }
// JTS: //Debug.println(e);
// JTS:     }
// JTS: //Debug.print(this);
// JTS: //Debug.printIfWatch(this);
// JTS:   }
// JTS:
// JTS:   private void computeEdgeEndLabels(BoundaryNodeRule boundaryNodeRule)
// JTS:   {
// JTS:     // Compute edge label for each EdgeEnd
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd ee = (EdgeEnd) it.next();
// JTS:       ee.computeLabel(boundaryNodeRule);
// JTS:     }
// JTS:   }
// JTS:
// JTS:   private int getLocation(int geomIndex, Coordinate p, GeometryGraph[] geom)
// JTS:   {
// JTS:     // compute location only on demand
// JTS:     if (ptInAreaLocation[geomIndex] == Location.NONE) {
// JTS:       ptInAreaLocation[geomIndex] = SimplePointInAreaLocator.locate(p, geom[geomIndex].getGeometry());
// JTS:     }
// JTS:     return ptInAreaLocation[geomIndex];
// JTS:   }
// JTS:
// JTS:   public boolean isAreaLabelsConsistent(GeometryGraph geomGraph)
// JTS:   {
// JTS:     computeEdgeEndLabels(geomGraph.getBoundaryNodeRule());
// JTS:     return checkAreaLabelsConsistent(0);
// JTS:   }
// JTS:
// JTS:   private boolean checkAreaLabelsConsistent(int geomIndex)
// JTS:   {
// JTS:     // Since edges are stored in CCW order around the node,
// JTS:     // As we move around the ring we move from the right to the left side of the edge
// JTS:     List edges = getEdges();
// JTS:     // if no edges, trivially consistent
// JTS:     if (edges.size() <= 0)
// JTS:       return true;
// JTS:     // initialize startLoc to location of last L side (if any)
// JTS:     int lastEdgeIndex = edges.size() - 1;
// JTS:     Label startLabel = ((EdgeEnd) edges.get(lastEdgeIndex)).getLabel();
// JTS:     int startLoc = startLabel.getLocation(geomIndex, Position.LEFT);
// JTS:     Assert.isTrue(startLoc != Location.NONE, "Found unlabelled area edge");
// JTS:
// JTS:     int currLoc = startLoc;
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS:       // we assume that we are only checking a area
// JTS:       Assert.isTrue(label.isArea(geomIndex), "Found non-area edge");
// JTS:       int leftLoc   = label.getLocation(geomIndex, Position.LEFT);
// JTS:       int rightLoc  = label.getLocation(geomIndex, Position.RIGHT);
// JTS: //System.out.println(leftLoc + " " + rightLoc);
// JTS: //Debug.print(this);
// JTS:       // check that edge is really a boundary between inside and outside!
// JTS:       if (leftLoc == rightLoc) {
// JTS:         return false;
// JTS:       }
// JTS:       // check side location conflict
// JTS:       //Assert.isTrue(rightLoc == currLoc, "side location conflict " + locStr);
// JTS:       if (rightLoc != currLoc) {
// JTS: //Debug.print(this);
// JTS:         return false;
// JTS:       }
// JTS:       currLoc = leftLoc;
// JTS:     }
// JTS:     return true;
// JTS:   }
// JTS:   void propagateSideLabels(int geomIndex)
// JTS:   {
// JTS:     // Since edges are stored in CCW order around the node,
// JTS:     // As we move around the ring we move from the right to the left side of the edge
// JTS:     int startLoc = Location.NONE ;
// JTS:
// JTS:     // initialize loc to location of last L side (if any)
// JTS: //System.out.println("finding start location");
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS:       if (label.isArea(geomIndex) && label.getLocation(geomIndex, Position.LEFT) != Location.NONE)
// JTS:         startLoc = label.getLocation(geomIndex, Position.LEFT);
// JTS:     }
// JTS:
// JTS:     // no labelled sides found, so no labels to propagate
// JTS:     if (startLoc == Location.NONE) return;
// JTS:
// JTS:     int currLoc = startLoc;
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       Label label = e.getLabel();
// JTS:       // set null ON values to be in current location
// JTS:       if (label.getLocation(geomIndex, Position.ON) == Location.NONE)
// JTS:           label.setLocation(geomIndex, Position.ON, currLoc);
// JTS:       // set side labels (if any)
// JTS:       if (label.isArea(geomIndex)) {
// JTS:         int leftLoc   = label.getLocation(geomIndex, Position.LEFT);
// JTS:         int rightLoc  = label.getLocation(geomIndex, Position.RIGHT);
// JTS:         // if there is a right location, that is the next location to propagate
// JTS:         if (rightLoc != Location.NONE) {
// JTS: //Debug.print(rightLoc != currLoc, this);
// JTS:           if (rightLoc != currLoc)
// JTS:             throw new TopologyException("side location conflict", e.getCoordinate());
// JTS:           if (leftLoc == Location.NONE) {
// JTS:             Assert.shouldNeverReachHere("found single null side (at " + e.getCoordinate() + ")");
// JTS:           }
// JTS:           currLoc = leftLoc;
// JTS:         }
// JTS:         else {
// JTS:           /** RHS is null - LHS must be null too.
// JTS:            *  This must be an edge from the other geometry, which has no location
// JTS:            *  labelling for this geometry.  This edge must lie wholly inside or outside
// JTS:            *  the other geometry (which is determined by the current location).
// JTS:            *  Assign both sides to be the current location.
// JTS:            */
// JTS:           Assert.isTrue(label.getLocation(geomIndex, Position.LEFT) == Location.NONE, "found single null side");
// JTS:           label.setLocation(geomIndex, Position.RIGHT, currLoc);
// JTS:           label.setLocation(geomIndex, Position.LEFT, currLoc);
// JTS:         }
// JTS:       }
// JTS:     }
// JTS:   }
// JTS:
// JTS:   public int findIndex(EdgeEnd eSearch)
// JTS:   {
// JTS:     iterator();   // force edgelist to be computed
// JTS:     for (int i = 0; i < edgeList.size(); i++ ) {
// JTS:       EdgeEnd e = (EdgeEnd) edgeList.get(i);
// JTS:       if (e == eSearch) return i;
// JTS:     }
// JTS:     return -1;
// JTS:   }
// JTS:
// JTS:   public void print(PrintStream out)
// JTS:   {
// JTS:     System.out.println("EdgeEndStar:   " + getCoordinate());
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       e.print(out);
// JTS:     }
// JTS:   }
// JTS:
// JTS:   public String toString()
// JTS:   {
// JTS:     StringBuffer buf = new StringBuffer();
// JTS:     buf.append("EdgeEndStar:   " + getCoordinate());
// JTS:     buf.append("\n");
// JTS:     for (Iterator it = iterator(); it.hasNext(); ) {
// JTS:       EdgeEnd e = (EdgeEnd) it.next();
// JTS:       buf.append(e);
// JTS:       buf.append("\n");
// JTS:     }
// JTS:     return buf.toString();
// JTS:   }
// JTS: }