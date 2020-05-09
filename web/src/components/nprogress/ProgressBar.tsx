import { useNProgress } from "@tanem/react-nprogress";
import React from "react";
import Bar from "./Bar";
import Container from "./Container";
import Spinner from "./Spinner";

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
            <Spinner />
        </Container>
    );
};

export default ProgressBar;
