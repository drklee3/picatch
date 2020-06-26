import React from "react";
import Breadcrumbs, { BreadcrumbsProps } from "./Breadcrumbs";
import Links from "./Links";

type HeaderProps = BreadcrumbsProps;

function Header(props: HeaderProps) {
    return (
        <header>
            <h3>picatch</h3>
            <Breadcrumbs {...props} />
            <Links />
        </header>
    );
}

export default Header;
