
    public enum AnimalType
    {
        Fish,
        Birds,
        Mammals
    }
    public interface IFly
    {
        void Fly();
    }

    public abstract class AnimalBase
    {

        public AnimalType Type { get; set; }

        public abstract void beforeEat();

        public  void Eat() {
            beforeEat();
        /// Do other logic
        }

        public virtual void AfterEat()
        {
            // Default Implementation
        }
    }


    public class Dog : AnimalBase, IFly
    {
        private string Name { get; set; }

        public Dog(string name, AnimalType type)
        {
            this.Name = name;
            this.Type = type;
        }
        
        public string WhatIsMyName()
        {
            return this.Name;
        }

        public override void beforeEat()
        {
            // Dog SHOULD implement this function
        }

        public void Fly()
        {
            // Dog SHOULD implement this function
        }

        // This is Optional 
        public override void AfterEat()
        {
            // Dog CAN implement/not implement this function
            base.AfterEat();
        }

        
    }

    class Program
    {
        public static void Main()
        {
            var dog = new Dog("Happi", AnimalType.Mammals);
            var name = dog.WhatIsMyName();
            dog.Fly();
        }
    }

