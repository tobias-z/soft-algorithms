import { QueryClient, QueryClientProvider, useQuery } from "@tanstack/react-query";
import { createContext, ReactNode, useContext, useEffect, useState } from "react";
import invariant from "tiny-invariant";
import { axios } from "./axios";

const queryClient = new QueryClient();

export default function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <RoadMap />
        </QueryClientProvider>
    );
}

async function getMap() {
    const res = await axios.get<Map>("/");
    return res.data;
}

type MapElementFns = {
    setRoadPosition?: (cb: (roadPosition: RoadPosition) => RoadPosition) => void;
};

type MapNode = MapElementFns & {
    id: number;
    roadName: string;
};

type MapRelation = MapElementFns & {
    nodeOne: MapNode;
    nodeTwo: MapNode;
    weight: number;
};

type Map = {
    nodes: Record<number, MapNode>;
    relations: Record<number, Array<MapRelation>>;
};

function RoadMap() {
    return (
        <div>
            <MapProvider>
                <Roads />
            </MapProvider>
        </div>
    );
}

type RoadPosition = {
    id: string;
    x: number;
    y: number;
    width: number;
    height: number;
    color: string;
    round?: boolean;
};

function getLength(weight: number) {
    console.log(weight * 100);
    return weight * 100;
}

const DEFAULT_ROAD_WIDTH = 5;

function Roads() {
    const map = useMapContext();
    const [lines, setLines] = useState<Array<RoadPosition>>([]);

    useEffect(() => {
        const visitedNodes = new Set<number>();
        const newLines: typeof lines = [];
        let currX = 10;
        let currY = 10;
        let currRoadName = "";
        let currWidth = 100;
        let currHeight = DEFAULT_ROAD_WIDTH;
        for (const node of Object.values(map.nodes)) {
            const relations = map.relations[node.id];
            node.setRoadPosition = cb => {
                setLines(lines =>
                    lines.map(line => {
                        if (line.id !== node.id.toString()) {
                            return line;
                        }
                        return cb(line);
                    })
                );
            };

            newLines.push({
                id: node.id.toString(),
                color: "blue",
                x: currX,
                y: currY,
                width: 20,
                height: 20,
                round: true,
            });

            for (const relation of relations) {
                const relationNode =
                    relation.nodeOne.id === node.id ? relation.nodeTwo : relation.nodeOne;
                relationNode.setRoadPosition = cb => {
                    setLines(lines =>
                        lines.map(line => {
                            const id = getRelationId(relation);
                            if (line.id !== id) {
                                return line;
                            }
                            return cb(line);
                        })
                    );
                };
                if (currHeight === DEFAULT_ROAD_WIDTH) {
                    // The last line we drew was sideways
                    if (node.roadName === relationNode.roadName) {
                        console.log(node, relationNode)
                        // Keep going the same way
                        newLines.push({
                            id: getRelationId(relation),
                            color: "black",
                            x: currX,
                            y: currY,
                            height: currHeight,
                            width: getLength(relation.weight),
                        });
                    } else {

                    }
                } else {
                }
            }
        }
        setLines(newLines);
    }, [map]);

    return (
        <div style={{ position: "relative" }}>
            {lines.map(line => (
                <>
                    <p>hello</p>
                    <div
                        key={`${line.x}-${line.y}`}
                        style={{
                            position: "absolute",
                            width: `${line.width}px`,
                            height: `${line.height}px`,
                            top: `${line.y}px`,
                            left: `${line.x}px`,
                            backgroundColor: line.color,
                        }}
                    />
                </>
            ))}
        </div>
    );
}

const MapContext = createContext<Map | null>(null);

function getRelationId(relation: MapRelation) {
    return `${relation.nodeOne.id}-${relation.nodeTwo.id}`;
}

function useMapContext() {
    const context = useContext(MapContext);
    if (!context) {
        throw new Error("useMapContext was used outside its provider");
    }
    return context;
}

type RCProps<T = unknown> = T & {
    children: ReactNode;
};

function MapProvider({ children }: RCProps) {
    const { data, isLoading, error } = useQuery({ queryKey: ["map"], queryFn: getMap });

    if (isLoading) {
        return <h1>Loading the map</h1>;
    }

    if (error) {
        return <h1>error {JSON.stringify(error)}</h1>;
    }

    invariant(data, "No map was found, but it should be here");

    return <MapContext.Provider value={data}>{children}</MapContext.Provider>;
}
