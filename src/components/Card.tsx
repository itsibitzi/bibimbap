import { FC } from "react";

type CardProps = {
  initialSide: "front" | "back";
  front: string;
  back: string;
};

export const Card: FC<CardProps> = () => {
  return (
    <div>
      <div>안녕하세요</div>
      <div>Hello</div>
    </div>
  );
};
