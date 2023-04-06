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
    const res = await axios.get<GraphMap>("/");
    return res.data;
}

type MapNode = {
    id: number;
    roadName: string;
};

type MapRelation = {
    nodeOne: MapNode;
    nodeTwo: MapNode;
    weight: number;
};

type GraphMap = {
    nodes: Record<number, MapNode>;
    relations: Record<number, Array<MapRelation>>;
};

type WeightedNode = {
    node: MapNode;
    prevNode?: WeightedNode;
    weight: number;
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
    isRoad: boolean;
    text?: string;
    onClick?: (oldLines: Record<string, RoadPosition>) => void;
};

function getLength(weight: number) {
    return weight * 25;
}

const DEFAULT_ROAD_WIDTH = 5;

function Roads() {
    const map = useMapContext();
    const [lines, setLines] = useState<Record<string, RoadPosition>>({});
    const [from, setFrom] = useState("");
    const [to, setTo] = useState("");
    const [road, setRoad] = useState<Array<WeightedNode>>([]);

    useEffect(() => {
        const newLines: typeof lines = {};
        function addLine(node: MapNode, currX: number, currY: number, currHeight: number) {
            const nodeId = node.id.toString();
            const relations = map.relations[Number(nodeId)];
            if (
                Object.hasOwn(newLines, node.id) &&
                !relations.find(relation => !Object.hasOwn(newLines, getRelationId(relation)))
            ) {
                return;
            }

            newLines[nodeId] = {
                id: node.id.toString(),
                color: "blue",
                x: currX - 6,
                y: currY - 6,
                width: 20,
                height: 20,
                isRoad: false,
                text: nodeId,
                onClick: oldLines => {
                    axios.get<Record<number, WeightedNode>>(`/${nodeId}`).then(res => {
                        const newLines = { ...oldLines };
                        for (const weightedNode of Object.values(res.data)) {
                            newLines[weightedNode.node.id].text = weightedNode.weight.toString();
                        }
                        setLines(newLines);
                    });
                },
            };

            const availableSpots = {
                right: true,
                top: true,
                bottom: true,
            };

            for (const relation of relations) {
                const id = getRelationId(relation);
                if (Object.hasOwn(newLines, id)) {
                    continue;
                }

                const relationNode =
                    relation.nodeOne.id === node.id ? relation.nodeTwo : relation.nodeOne;

                function onRelationClick(oldLines: Record<number, RoadPosition>) {
                    axios.get<Record<number, WeightedNode>>(`/${relationNode.id}`).then(res => {
                        const newLines = { ...oldLines };
                        for (const weightedNode of Object.values(res.data)) {
                            newLines[weightedNode.node.id].text = weightedNode.weight.toString();
                        }
                        setLines(newLines);
                    });
                }

                let width = 0;
                let nextX = currX;
                let nextY = currY;
                let after: () => void = () => { };
                if (availableSpots.right) {
                    availableSpots.right = false;
                    width = getLength(relation.weight);
                    currHeight = DEFAULT_ROAD_WIDTH;
                    newLines[relationNode.id.toString()] = {
                        id: relationNode.id.toString(),
                        x: nextX + width - 6,
                        y: nextY - 6,
                        height: 20,
                        width: 20,
                        color: "blue",
                        isRoad: false,
                        onClick: onRelationClick,
                        text: relationNode.id.toString(),
                    };
                    after = () => addLine(relationNode, nextX + width, nextY, currHeight);
                } else if (availableSpots.top) {
                    availableSpots.top = false;
                    currHeight = getLength(relation.weight);
                    width = DEFAULT_ROAD_WIDTH;
                    nextY = nextY - currHeight;
                    newLines[relationNode.id.toString()] = {
                        id: relationNode.id.toString(),
                        x: nextX - 6,
                        y: nextY - 6,
                        height: 20,
                        width: 20,
                        color: "blue",
                        isRoad: false,
                        onClick: onRelationClick,
                        text: relationNode.id.toString(),
                    };
                    after = () => addLine(relationNode, nextX, nextY, currHeight);
                } else if (availableSpots.bottom) {
                    availableSpots.bottom = false;
                    currHeight = getLength(relation.weight);
                    width = DEFAULT_ROAD_WIDTH;
                    newLines[relationNode.id.toString()] = {
                        id: relationNode.id.toString(),
                        x: nextX - 6,
                        y: nextY + currHeight - 6,
                        height: 20,
                        width: 20,
                        color: "blue",
                        isRoad: false,
                        onClick: onRelationClick,
                        text: relationNode.id.toString(),
                    };
                    after = () => addLine(relationNode, nextX, nextY + currHeight, currHeight);
                }

                newLines[id] = {
                    id,
                    color: "black",
                    x: nextX,
                    y: nextY,
                    height: currHeight,
                    width,
                    isRoad: true,
                    text: relationNode.roadName,
                };
                after();
            }
        }
        const currX = 50;
        const currY = 700;
        const currHeight = 0;
        for (let node of Object.values(map.nodes)) {
            addLine(node, currX, currY, currHeight);
        }
        setLines(newLines);
    }, [map]);

    return (
        <div style={{ position: "relative" }}>
            <>
                <form
                    onSubmit={e => {
                        e.preventDefault();
                        axios
                            .get<Record<number, WeightedNode>>(`/${from}`)
                            .then(res => {
                                const data = res.data;
                                if (!Object.hasOwn(data, Number(to))) {
                                    return;
                                }
                                const road: Array<WeightedNode> = [];
                                function appendNext(weightedNode: WeightedNode) {
                                    road.push(weightedNode);
                                    if (weightedNode.prevNode) {
                                        appendNext(weightedNode.prevNode)
                                    }
                                }
                                const startingNode = data[Number(to)];
                                appendNext(startingNode);
                                setRoad(road.reverse());
                            })
                            .catch(() => { });
                    }}
                >
                    <input name="from" type="number" onChange={e => setFrom(e.target.value)} />
                    <input name="to" type="number" onChange={e => setTo(e.target.value)} />
                    <button type="submit">Check</button>
                </form>
                {road.map(node => (
                    <p key={node.node.id}>{node.node.id} ({node.node.roadName}) - takes {node.weight} min</p>
                ))}
                {Object.values(lines).map(line => (
                    <div
                        key={line.id}
                        style={{
                            position: "absolute",
                            width: `${line.width}px`,
                            height: `${line.height}px`,
                            top: `${line.y}px`,
                            left: `${line.x}px`,
                            backgroundColor: line.color,
                            borderRadius: !line.isRoad ? "10px" : "",
                            zIndex: !line.isRoad ? "100" : "50",
                            textAlign: "center",
                            color: "white",
                        }}
                        onClick={() => {
                            if (line.onClick) {
                                line.onClick(lines);
                            }
                        }}
                    >
                        {!line.isRoad ? line.text : <p style={{ color: "black" }}>{line.text}</p>}
                    </div>
                ))}
            </>
        </div>
    );
}

const MapContext = createContext<GraphMap | null>(null);

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
