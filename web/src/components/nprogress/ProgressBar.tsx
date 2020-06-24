import { useNProgress } from "@tanem/react-nprogress";
import React from "react";
import Bar from "./Bar";
import Container from "./Container";

type ProgressBarProps = {
    isAnimating?: boolean;
};

const ProgressBar = ({ isAnimating }: ProgressBarProps) => {
    const { animationDuration, isFinished, progress } = useNProgress({
        isAnimating,
    });

    return (
        <Container
            animationDuration={animationDuration}
            isFinished={isFinished}
        >
            <Bar animationDuration={animationDuration} progress={progress} />
        </Container>
    );
};

export default ProgressBar;
