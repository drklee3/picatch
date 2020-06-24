import React, { useState, useEffect } from "react";

type Props = React.ImgHTMLAttributes<HTMLImageElement> & {
    setIsLoading?: (loading: boolean) => void;
};

function ImageItem(props: Props) {
    const [loaded, setLoaded] = useState(false);
    const { setIsLoading, ...rest } = props;

    useEffect(() => {
        if (setIsLoading) {
            setIsLoading(true);
        }
    }, []);

    const onLoad = () => {
        setLoaded(true);

        if (setIsLoading) {
            setIsLoading(false);
        }
    };

    return (
        <img
            {...rest}
            alt={props.alt ? props.alt : "Image"}
            className={`${props.className || ""} ${loaded ? "visible" : ""}`}
            onLoad={onLoad}
        />
    );
}

export default ImageItem;
