import { FC } from "react";

type MenuButtonProps = {
  text: string;
  onClick: () => void;
};

export const MenuButton: FC<MenuButtonProps> = ({ text, onClick }) => {
  return <button onClick={onClick}>{text}</button>;
};
