import React from "react";
import Breadcrumbs, { BreadcrumbsProps } from "./Breadcrumbs";
import { ActiveFileActions } from "../reducers/activeFileActions";
import Links from "./Links";

type HeaderProps = BreadcrumbsProps;

function Header(props: HeaderProps) {
    function navigateHome() {
        props.dispatch({
            type: ActiveFileActions.SET_ALBUM,
            album: "/",
        });
    }

    return (
        <header>
            <div>
                <h3 onClick={navigateHome}>picatch</h3>
            </div>
            <Breadcrumbs {...props} />
            <Links />
        </header>
    );
}

export default Header;
