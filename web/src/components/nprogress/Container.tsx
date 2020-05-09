import React from "react";

type ContainerTypes = {
    animationDuration: number;
    children: React.ReactNode;
    isFinished: boolean;
};

const Container = ({
    children,
    isFinished,
    animationDuration,
}: ContainerTypes) => (
    <div
        style={{
            opacity: isFinished ? 0 : 1,
            pointerEvents: "none",
            transition: `opacity ${animationDuration}ms linear`,
        }}
    >
        {children}
    </div>
);

export default Container;
