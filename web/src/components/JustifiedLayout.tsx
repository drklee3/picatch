// Based off https://github.com/Dean177/react-justified-layout

import justifiedLayout from "justified-layout";
import React from "react";

interface Dimensions {
    width: number;
    height: number;
}

interface BaseChildProps {
    // Allow any other props
    [x: string]: any;
}

interface AspectRatioChildProps extends BaseChildProps {
    aspectRatio: number;
}

interface DimensionsChildProps extends BaseChildProps {
    style: { width: number; height: number };
}

type JustifiedChildProps = AspectRatioChildProps | DimensionsChildProps;

function isReactElement(e: React.ReactChild): e is React.ReactElement {
    return (e as React.ReactElement).props !== undefined;
}

function extractDimension(node: React.ReactChild): Dimensions {
    if (!isReactElement(node)) {
        return { width: 0, height: 0 };
    }

    const { props } = node;

    if (!props) {
        // TODO
        return { width: 0, height: 0 };
    }

    return { height: props.style.height, width: props.style.width };
    // TODO Measure the element if no props are here?
}

function normalizeDimension(dimension: Dimensions): number {
    if (dimension.height && dimension.width) {
        return dimension.width / dimension.height;
    }

    return 0;
}

// See http://flickr.github.io/justified-layout/ for an explanation of what these props do
interface Props {
    boxSpacing?: number | { horizontal: number; vertical: number };

    // React children in general can be React.ReactChildren, but since we require
    // certain props to be added, it can't be other types e.g. fragment
    // Alternatively could probably filter them out, but then they won't show up anyways
    children?:
        | React.ReactElement<JustifiedChildProps>
        | React.ReactElement<JustifiedChildProps>[];
    containerPadding?:
        | number
        | { bottom: number; left: number; right: number; top: number };
    containerWidth?: number;
    forceAspectRatio?: boolean | number;
    fullWidthBreakoutRowCadence?: boolean | number;
    maxNumRows?: number;
    showWidows?: boolean;
    targetRowHeight?: number;
    targetRowHeightTolerance?: number;
    widowLayoutStyle?: "left" | "justify" | "center" | undefined;
    style?: React.CSSProperties;
}

const defaultConfig: Props = {
    boxSpacing: 10,
    containerPadding: 10,
    containerWidth: 1060,
    forceAspectRatio: false,
    fullWidthBreakoutRowCadence: false,
    maxNumRows: Number.POSITIVE_INFINITY,
    showWidows: true,
    targetRowHeight: 320,
    targetRowHeightTolerance: 0.25,
    widowLayoutStyle: "left",
};

function JustifiedLayout(props: Props) {
    const { children, style, ...config } = props;
    const { containerWidth } = props;

    if (!children) {
        return null;
    }

    const childDims = React.Children.map(children, extractDimension).map(
        normalizeDimension
    );

    const { containerHeight, boxes } = justifiedLayout(childDims, {
        ...defaultConfig,
        ...config,
    });

    // Ensure children is an array so we can slice/map it
    const childrenArray = children instanceof Array ? children : [children];

    // Slice then "zip", not sure if we really need the slice?
    const elementLayout = childrenArray.slice(0, boxes.length).map((e, i) => {
        return { element: e, layoutBox: boxes[i] };
    });

    return (
        <div
            style={{
                position: "relative",
                ...style,
                height: containerHeight,
                width: containerWidth,
            }}
        >
            {elementLayout.map(({ element, layoutBox }) => {
                const { height, left, top, width } = layoutBox;

                return React.cloneElement(element, {
                    ...element.props,
                    style: {
                        ...element.props.style,
                        position: "absolute",
                        height,
                        left,
                        top,
                        width,
                    },
                });
            })}
        </div>
    );
}

export default JustifiedLayout;
